# decode_scan_mcu_multiply_overflow

`Decoder::decode_scan`'s MCU-grid bounds checks (`src/decoder.rs:911`, `:916`, `:1025`) multiply a
u16 loop counter (`mcu_x`/`mcu_y`) by the literal `8` and compare directly against
`frame.image_size.{width,height}` (also `u16`), with no widening. GitHub issue #283 (open, filed
after a fuzz run with overflow checks enabled) reports three separate "attempt to multiply with
overflow" panics at exactly these lines, with three attached repro inputs and stack traces. Picked
from `IDEAS.md`/the task brief to re-verify against current HEAD (base_commit eb2d7c0) and close
the loop with a real repro + fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-run the issue's own three attached fuzzer-found input files against current HEAD, driven through the real public API (`jpeg_decoder::Decoder::new(data).decode()`), debug-assertions build | `cargo +1.87.0 run --example repro283 -- <file>` (dev profile, overflow-checks on by default), one run per file | All three inputs still panic at the exact same lines the issue reports (911, 916, 1025), unchanged from the original 2025-05 fuzz report - manual boundary-value/arithmetic reasoning (see Root cause) confirms the same shape of bug is still present in code at those lines |
| Repeat each of the three inputs 3x to check determinism | Same debug build, same inputs, 3 runs each | 3/3 for all three inputs, byte-identical panic location every time - fully deterministic (no randomness anywhere in decode of a fixed byte stream) |

Given the issue's own attached inputs reproduced immediately and deterministically against current
HEAD on the first attempt, no further escalation (miri/ASAN/a fresh fuzz harness) was needed - this
is a straightforward, fully deterministic arithmetic-overflow class of bug, already isolated to
three specific lines by the reporter's own fuzzer, and confirmed still present by direct reading of
the arithmetic (see Root cause) plus the re-run.

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs for all three inputs, debug build
(`cargo +1.87.0 build`, dev profile - overflow checks on by Cargo's own default for that profile),
same panic location every time.

**Fully reproducible**: **Yes**. `repro/src/main.rs` feeds each of the three real fuzzer-found
input files (bundled in `repro/inputs/`, taken from the issue's own attached zip) straight through
the crate's real, public `jpeg_decoder::Decoder::new(data).decode()` entry point end to end - the
same call path a genuine caller of this decode-only library would use. There is no second
in-process participant to fake here (the finding is about how the decoder's own bounds-check
arithmetic reacts to a byte stream it was handed, not about a peer/caller relationship) - see
`CONFIRMATION.md` section 11.

## Class

crash (panic on malformed/adversarial input under debug-assertions / overflow-checks builds; not
a memory-safety issue in a release build, since Rust's default release profile wraps instead of
panicking, but still a DoS-shaped crash for any consumer that ships debug-assertions or
`overflow-checks = true`, and evidence of an unvalidated invariant either way)

## Root cause

`Decoder::decode_scan`'s interleaved/non-interleaved MCU-grid loop bounds mcu_x/mcu_y against the
image dimensions with (`src/decoder.rs:911,916,1025` at base_commit eb2d7c0):

```rust
for mcu_y in 0..max_mcu_y {
    if mcu_y * 8 >= frame.image_size.height {   // :911
        break;
    }
    for mcu_x in 0..max_mcu_x {
        if mcu_x * 8 >= frame.image_size.width {  // :916
            break;
        }
        ...
    }
    ...
    if !is_interleaved
        && (mcu_y + 1) * 8 < frame.image_size.height   // :1025
        ...
```

`mcu_x`/`mcu_y` are `u16` (ranging over `max_mcu_x`/`max_mcu_y`, which for a non-interleaved scan
is `components[i].block_size.width`/`.height`, itself `u16` - see `src/parser.rs`'s `Dimensions`
struct and `update_component_sizes`). `frame.image_size.width`/`.height` are also `u16`. The
literal `8` is inferred as `u16` from context, so `mcu_x * 8` is a `u16 * u16` multiply that
overflows once `mcu_x >= 8192` (`8192 * 8 == 65536`, one past `u16::MAX`).

`component.block_size.width` is computed as `mcu_size.width * component.horizontal_sampling_factor`,
where `mcu_size.width = ceil_div(image_width, h_max * 8)` (`h_max` = the largest horizontal
sampling factor across *all* components in the frame). When a component's own sampling factor
equals `h_max` but doesn't evenly divide the image width, the ceiling rounding in `mcu_size.width`
combined with the subsequent multiply by `h_max` can push `block_size.width` (or `.height`) past
what a naive `ceil(image_width / 8)` would give - e.g. for a large image width with a sampling
factor that doesn't divide evenly, `block_size.width` can land at 8193 or more, one past the safe
`u16` boundary for `* 8`. The non-interleaved branch (used for lossless/progressive per-component
scans, or any single-component scan) then iterates `mcu_x`/`mcu_y` up to `block_size.{width,height}
- 1`, which reaches the overflowing value before the bounds check itself has a chance to `break`
the loop - the overflow happens while *evaluating* the very break condition meant to guard against
it, so there's no valid input for which the check "arrives too late"; it's structurally impossible
for the u16 arithmetic to protect itself.

All three panic sites are the same arithmetic pattern in the same function, differing only in which
specific comparison (`mcu_y`'s row bound, `mcu_x`'s column bound, or the MCU-row-flush look-ahead
check `(mcu_y + 1) * 8 < height` used to decide whether to defer sending a partially-built
non-interleaved row to the worker thread) trips first for a given crafted input's dimensions and
sampling factors.

## Introduced by

6d9fd3e "The decoder now treats non-interleaved streams as blocks" (2021-01-14). Verified with
`git log -S"mcu_y * 8 >= frame.image_size.height" -- src/decoder.rs`,
`git log -S"mcu_x * 8 >= frame.image_size.width" -- src/decoder.rs`, and
`git log -S"(mcu_y + 1) * 8 < frame.image_size.height" -- src/decoder.rs` - all three land on this
same commit, which introduced the non-interleaved MCU-grid iteration (and its `block_size`-based
loop bounds) alongside the interleaved path's pre-existing `mcu_size`-based bounds.

## The fix

Widen both operands to `u32` before multiplying/comparing at all three sites, so the multiply
cannot overflow for any `u16` input (max possible value `65535 * 8 = 524280`, far inside `u32`):

```rust
if mcu_y as u32 * 8 >= frame.image_size.height as u32 { ... }      // :911
if mcu_x as u32 * 8 >= frame.image_size.width as u32 { ... }       // :916
&& (mcu_y as u32 + 1) * 8 < frame.image_size.height as u32          // :1025
```

(The `(mcu_y + 1) % component.vertical_sampling_factor as u16` on the same line as the :1025 fix is
left as `u16` arithmetic - `mcu_y` never exceeds `max_mcu_y - 1 <= u16::MAX - 1`, so `mcu_y + 1`
cannot itself overflow `u16`; only the `* 8` needed widening.)

This is a minimal, purely-arithmetic fix: it doesn't change which inputs are accepted or rejected,
only how the bound is computed, so no decode-error/success behavior changes for any input that
didn't previously panic.

Re-running `repro/` against the patched tree: all three previously-panicking inputs now return
either `Ok` (fully decoded) or a normal `Err(...)` (`UnexpectedEof`, `Unsupported(Hierarchical)`) -
see `output.txt` for the exact before/after output, captured by actually running `just test` end to
end from a fresh clone in a scratch directory.

- Project test suite (`cargo +1.87.0 test --all-features`, main worktree): 23 passed / 0 failed
  before the patch (5 unit tests + 12 in `tests/lib.rs`'s `crashtest` + 1 each in
  `tests/rayon.rs`/`rayon-0.rs`/`rayon-1.rs` + 1 in `rayon-2.rs` + 2 doc-tests), 23 passed / 0
  failed after - identical breakdown both times, no new failures, no regressions.

## What's not done / open questions

- Only the three specific inputs from the issue's own fuzz run were used to confirm the bug and
  the fix; a broader fuzz/boundary sweep across other sampling-factor combinations that could also
  push `block_size.width`/`.height` past 8192 wasn't run (the reporter's fuzzer already covers this
  space better than a short one-off harness would this session, and the fix is a general
  arithmetic-widening fix, not input-specific).
- Toolchain note: this sandbox's pinned `rust-toolchain` (1.61.0) fails to build the current
  `Cargo.lock`'s `libm` dependency (needs rustc >= 1.63); all builds/tests in this session used
  `cargo +1.87.0` explicitly instead. Recorded here since a future session re-running this
  candidate's `repro/` or the main project's test suite will hit the same thing.
- Did not check whether the same u16-multiply-by-8 pattern exists anywhere else in the codebase
  outside `decode_scan` (e.g. `decoder/lossless.rs`'s own MCU loop, if any) - out of scope for this
  candidate (one bug/pattern-instance per candidate), but worth a follow-up grep.
