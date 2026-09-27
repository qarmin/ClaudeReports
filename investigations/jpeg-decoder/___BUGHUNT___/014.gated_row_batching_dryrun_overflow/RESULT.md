# gated_row_batching_dryrun_overflow

Per this session's task ("perf_optimize_candidates_adversarial_input_check" - check whether
`___OPTIMIZATIONS___/` candidates that did correctness-sensitive rework introduce new panics/OOB
on malformed input, since they were only verified for perf correctness on presumably well-formed
inputs), this candidate differentially ran every distinct `___OPTIMIZATIONS___/` `patch.diff`
against a corpus of 323 malformed/adversarial/reference JPEG files (`tests/crashtest/images/`,
`fuzz-afl/in/`, `tests/reftest/images/`) through the crate's real public `Decoder::decode()` API,
comparing outcomes (Ok+hash / Err+message / catch_unwind Panic+message) against the pristine
baseline. `009.gated_row_batching` and `016.hybrid_gated_periodic_flush` (which embeds the same
code) both introduce a new debug-assertions panic on `tests/crashtest/images/max_size.jpg`, an
existing crashtest fixture that decodes cleanly (a plain `Err`) on both pristine `HEAD` and every
other checked `___OPTIMIZATIONS___` patch.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Differential sweep: standalone `examples/diffcheck.rs` harness (built once, added temporarily to the worktree, reverted before finishing) decodes every file under three corpus directories via `jpeg_decoder::Decoder::decode()` wrapped in `std::panic::catch_unwind`, printing a stable per-file outcome line (`OK:<len>:<FNV1a hash>` / `ERR:<msg>` / `PANIC:<msg>`) | Debug build (`cargo +stable build --example diffcheck`, default dev profile: `debug-assertions`/`overflow-checks` on), 323 files total (111 `tests/crashtest/images`, 104 `fuzz-afl/in` AFL seed corpus, 108 `tests/reftest/images`), run once per candidate patch (`002`, `003`, `004`, `005`, `006`, `007`, `008`, `009`, `013`, `016` - the ones with a real `patch.diff`; `001`/`010`/`014`/`015` have no patch, per their own RESULT.md, so nothing to check) applied individually to an otherwise-pristine `eb2d7c0` worktree, output diffed against a baseline sweep of the same 323 files on unpatched `eb2d7c0` | Baseline: 0 panics, 0 errors-that-shouldn't-be-errors across all 323 files. `002`/`004`/`005`/`006`/`007`/`008`/`013`: byte-identical outcome for all 323 files vs. baseline (no divergence found). `003.raise_lut_bits` (`LUT_BITS: 8 -> 10`): `dc-predictor-overflow.jpg` still decodes `Ok` both ways but with **different output pixel bytes** (hash `76e6108a25adb502` baseline vs `fc11a4f8bc0cc426` patched, same length) - a real but narrower, separately-noted divergence on a wrapping-arithmetic-dependent adversarial fixture; not pursued further this session, see "New suspects noticed" below since scope permitted only one full dig. **`009.gated_row_batching` and `016.hybrid_gated_periodic_flush`: `max_size.jpg` changes from `ERR:invalid JPEG format: failed to decode huffman code` (baseline) to `PANIC:attempt to multiply with overflow`** - both patches contain byte-for-byte the same newly-added pre-scan `batch_this_scan` dry-run loop, so both hit the identical bug. |
| Root-cause via `RUST_BACKTRACE=full` on a minimal `examples/single_test.rs` (reads a path, calls `decoder.decode()`, prints `Ok`/`Err`) built against the 009-patched worktree | Debug build, `tests/crashtest/images/max_size.jpg` (186 bytes; SOF0 declaring `width=height=0xFFFF`, single component, garbage/truncated entropy-coded scan) | Panic at `src/decoder.rs:936:32`, `attempt to multiply with overflow`, inside `decode_scan`'s newly-added `batch_this_scan` pre-scan loop - see "Root cause" below. Reproduced deterministically 3/3 runs. |
| Confirmation against the project's own existing test suite (not just the ad hoc harness): `cargo +stable test` (default `dev` profile, not `--release`) with `009`'s `patch.diff` applied | Debug build, full workspace test suite | `tests/lib.rs`'s pre-existing `crashtest::crashtest` test (iterates every file in `tests/crashtest/images/` through the real public API, no `catch_unwind`) genuinely **fails**: `11 passed; 1 failed`, with the exact same panic message and location. Confirms this is not an artifact of the ad hoc harness - the project's own real test infrastructure, run in the same profile a real `cargo test` invocation would use, already catches this once the patch is applied. Perf-optimize's own verification for `009`/`016` ran `cargo +stable test --release` (per those candidates' own `RESULT.md`), which silently masks the overflow (see "Root cause"/build-dependence note below) - that is why it wasn't caught during the original perf work. |

Given the second and third approaches reproduced the exact same panic deterministically via two
independent paths (a standalone repro and the project's own pre-existing test), the soft cap of 3
escalating approaches was reached with a clean, root-caused, reliably-reproducing signal - no
further tool escalation (miri/ASAN/fuzzing) was needed for what is a straightforward,
deterministic integer-overflow-on-malformed-input bug.

## Status

**Confirmed.** Reproduces deterministically, 3/3 runs, both via the standalone `repro/` crate
(release build with `overflow-checks`/`debug-assertions` forced on - see build-dependence note
below) and via the project's own `cargo +stable test` (plain `dev` profile) `crashtest::crashtest`
test. Not layout/seed-dependent - purely a function of the fixed input bytes and the patched
source.

**Fully reproducible:** Yes. `repro/` drives the real, public `jpeg_decoder::Decoder::decode()`
entry point with a real on-disk file (`tests/crashtest/images/max_size.jpg`, bundled verbatim as
`repro/max_size.jpg`) that already exists in this project's own crashtest corpus specifically to
exercise `width=height=0xFFFF` edge cases - a single external artifact with no second
participant to fake (`CONFIRMATION.md` section 11's "no second participant" case).

**Build dependence** (`CONFIRMATION.md` section 2): reproduces under `debug-assertions`/
`overflow-checks` (the crate's own default `dev`/`test` profile, and this candidate's `repro/`
which forces these on even in `--release`). Under a plain `--release` build (`overflow-checks`
off, the default), the multiply silently wraps instead of panicking - `decode()` still returns the
identical clean `Err` either way for this specific input, since the wrapped `total_rows` value
happens not to change the final decode outcome for this particular malformed file (decode fails
via the ordinary "invalid huffman code" path regardless of `batch_this_scan`'s value). This does
not make the bug less real: it is precisely the debug-assertions build that perf-optimize's own
`RESULT.md` for `009`/`016` did *not* use for its correctness check (`cargo +stable test
--release`), which is why this was not caught during the original perf-optimize session.

## Class

crash

## Root cause

`009.gated_row_batching/patch.diff` (and, identically, `016.hybrid_gated_periodic_flush/patch.diff`,
which embeds the exact same code) adds a "dry run" loop in `Decoder::decode_scan` that runs
*before* any entropy decoding starts, to decide up front whether the scan is large enough to
batch through `Worker::append_rows` instead of one `append_row` call per MCU row:

```rust
let batch_this_scan = {
    let mut total_rows = 0usize;
    for mcu_y in 0..max_mcu_y {
        if mcu_y * 8 >= frame.image_size.height {
            break;
        }
        for (i, component) in components.iter().enumerate() {
            if finished[i] {
                if !is_interleaved
                    && (mcu_y + 1) * 8 < frame.image_size.height   // <- src/decoder.rs:936
                    && (mcu_y + 1) % component.vertical_sampling_factor as u16 > 0
                {
                    continue;
                }
                total_rows += 1;
            }
        }
    }
    total_rows > ROW_BATCH_THRESHOLD
};
```

`mcu_y` (from `max_mcu_y`, itself `components[0].block_size.height`/`frame.mcu_size.height`) and
`frame.image_size.height` are both `u16` (`src/parser.rs`'s `Dimensions` struct). The exact same
expression, `(mcu_y + 1) * 8 < frame.image_size.height`, already exists verbatim in the
pre-existing baseline code (unchanged by any of these patches, at `src/decoder.rs:1025` in
pristine `eb2d7c0`, inside the real per-row "send the coefficients to the worker" loop) - but in
the *real* loop this is never reached for a genuinely malformed/truncated bitstream, because the
actual entropy-decode call (`huffman.decode(...)?`) a few lines earlier fails and returns `Err`
via `?` on essentially the first MCU for a file this small and this garbled, long before `mcu_y`
could ever grow large enough to overflow.

The new dry-run loop has no such protection: it never touches the entropy-coded bitstream at all,
it only walks `0..max_mcu_y` using frame/component metadata (`finished`, sampling factors,
`is_interleaved`) that is fully attacker-controlled via the SOF header, with **no early exit tied
to whether decoding would actually succeed**. For `max_size.jpg` (`SOF0`: `width=height=0xFFFF`,
one component, `h=v=1`), `max_mcu_y = ceil(65535/8) = 8192`; at `mcu_y = 8191` (which passes the
loop's own `mcu_y * 8 >= frame.image_size.height` break-check, since `8191*8 = 65528 < 65535`),
the inner check evaluates `(8191 + 1) * 8 = 8192 * 8 = 65536` as a `u16` multiply, overflowing
`u16::MAX` (65535) and panicking under `debug-assertions`/`overflow-checks` (silently wrapping to
0 otherwise). The dry-run loop reaches this point unconditionally on every call, regardless of
whether the scan's actual entropy-coded data is valid, truncated, or garbage - which is exactly
the "adversarial-input, not perf-input" gap this candidate was checking for: perf-optimize's own
verification measured wall-clock time and *correctness on images that decode successfully*, never
exercising this pre-scan dry run against a file crafted purely to probe SOF-header boundary values.

## Introduced by

Not applicable in the usual sense - this is a regression in unmerged, always-reverted
`___OPTIMIZATIONS___/009.gated_row_batching/patch.diff` and
`___OPTIMIZATIONS___/016.hybrid_gated_periodic_flush/patch.diff` (bundled here as
`009_under_review.diff`/`016_under_review.diff`), not in any commit of this project's own tracked
git history - neither patch has ever been applied to the real working tree except transiently
during perf-optimize's own measurement sessions (always reverted afterward, per that skill's own
rules). The pre-existing, unmodified expression these patches copy verbatim
(`(mcu_y + 1) * 8 < frame.image_size.height`) was itself introduced by 6d9fd3e "The decoder now
treats non-interleaved streams as blocks" (found via `git log -S"(mcu_y + 1) * 8" -- src/decoder.rs`)
- but that pre-existing copy is not itself buggy in context (see "Root cause" above for why the
real loop never reaches the overflow), so 6d9fd3e is not "the" introducing commit for the actual
regression; the regression is specific to lifting that expression into a new, unguarded pre-scan
context, which only exists in the two patch files named above.

## The fix

Widen the two multiplications inside the dry-run loop to `u32` before comparing, mirroring the
same width the real per-row loop's own bounds arithmetic elsewhere in `decode_scan` effectively
enjoys via its interleaving with checked entropy-decode calls (this fix does not touch that
pre-existing real-loop expression at all - it is out of scope for this candidate, since it isn't
what's newly broken; see "What's not done" below):

```rust
for mcu_y in 0..max_mcu_y {
    if mcu_y as u32 * 8 >= frame.image_size.height as u32 {
        break;
    }
    for (i, component) in components.iter().enumerate() {
        if finished[i] {
            if !is_interleaved
                && (mcu_y as u32 + 1) * 8 < frame.image_size.height as u32
                && (mcu_y + 1) % component.vertical_sampling_factor as u16 > 0
            {
                continue;
            }
            total_rows += 1;
        }
    }
}
```

See `patch.diff` (+7/-2 lines, applies cleanly on top of *either* `009_under_review.diff` or
`016_under_review.diff` - the affected code block is byte-for-byte identical in both patches, only
its surrounding comments differ).

Verified:
- `repro/` against `009`'s patch alone: panics 3/3 runs (`output.txt`). Against `009`'s patch +
  this `patch.diff`: no longer panics, 3/3 runs - `decode()` returns the same clean
  `Err("invalid JPEG format: failed to decode huffman code")` as pristine baseline, byte-for-byte
  identical message.
- Same before/after re-checked against `016`'s patch instead of `009`'s (identical code path,
  identical fix applies cleanly) - same result: panics before, clean `Err` after.
- Re-ran the full 323-file differential sweep with `009`'s patch + this `patch.diff` applied:
  **0 diffs vs. pristine baseline** across all 323 files (confirms the fix doesn't just narrowly
  patch this one file - the widened arithmetic can't overflow for any `u16` `mcu_y`/height
  combination, and no other file's outcome changed).
- Project test suite, debug profile (`cargo +stable test`), base_commit eb2d7c0:
  - Pristine (no patch): **23 passed, 0 failed** (5 unit + 12 lib-integration incl. `crashtest` +
    4 rayon integration + 2 doctests).
  - `009`'s patch alone, no fix: `crashtest::crashtest` **FAILED** (panic, `11 passed; 1 failed`
    in that test binary) - a new failure not present in the pristine baseline.
  - `009`'s patch + this `patch.diff`: **23 passed, 0 failed** - identical to pristine baseline,
    `crashtest::crashtest` passes again.

## What's not done / open questions

- **New suspect, not investigated further here** (out of scope - this candidate's own soft cap of
  3 approaches was reached on the `009`/`016` finding before this could be dug into): candidate
  `003.raise_lut_bits`/`014.lut_bits_on_progressive` (`LUT_BITS: 8 -> 10` in `src/huffman.rs`)
  changes the *decoded pixel bytes* (not just an error vs. a crash) for
  `tests/crashtest/images/dc-predictor-overflow.jpg` - both builds return `Ok` with the same
  output length but a different hash, meaning the actual pixel values differ. This is a smaller,
  narrower kind of divergence than a new panic, but it is still "different output for the same
  malformed input," which is exactly this candidate's stated risk. Worth a dedicated follow-up
  candidate: likely related to how a wider `LUT_BITS` changes which Huffman-decode path
  (LUT-hit vs. the manual `maxcode`/`delta`/`values` fallback in `HuffmanDecoder::decode`)
  gets used for the specific bit patterns this adversarial fixture's `wrapping_add`-triggering
  huffman codes happen to produce, interacting with `dc-predictor-overflow.jpg`'s own
  known-intentional DC-predictor-overflow behavior (see the comment above the `wrapping_add` call
  site in `src/decoder.rs` referencing this exact file).
- The same "unguarded pre-scan dry run over attacker-controlled `u16` metadata" pattern was
  checked only for `009`/`016`'s specific `batch_this_scan` loop - `013.periodic_row_batch_flush`
  (the third row-batching variant) does not have a pre-scan dry run at all (confirmed: no
  divergence found, and its own diff has no equivalent loop), so it is not affected by this class
  of bug. No other `___OPTIMIZATIONS___` candidate was found to add a similar pre-scan/dry-run
  construct.
- This candidate's fix intentionally does not touch the pre-existing, unmodified real-loop copy of
  the same expression at pristine `src/decoder.rs:1025` (see "Root cause") - it is not part of
  either patch under review, not currently reachable with a `?`-short-circuiting decode failure in
  any input tried this session, and touching it would be a separate, baseline-scoped candidate
  (not what this candidate's patch is scoped to fix). Whether a sufficiently crafted *valid-looking*
  (all-zero, marker-terminated, then infinitely zero-filled per `HuffmanDecoder::read_bits`'s own
  EOF zero-fill behavior) bitstream could still drive the *real* loop to `mcu_y` values large
  enough to hit the identical overflow was not tested this session - flagged as a possible
  pre-existing (not perf-optimize-introduced) latent issue worth its own candidate.
