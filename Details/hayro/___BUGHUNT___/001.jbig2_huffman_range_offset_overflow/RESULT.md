# jbig2_huffman_range_offset_overflow

`hayro-jbig2/src/huffman_table.rs:455` (`HuffmanNode::decode_from`) computes a decoded Huffman
value as `leaf.range_low + range_offset` (or `- range_offset` for a "lower" table line), where
`range_offset` is read straight off the attacker-controlled JBIG2 bitstream via
`reader.read_bits(leaf.range_length)`, with no checked/wrapping arithmetic. Pre-session triage
found all 17 stale `hayro-fuzz/artifacts/fuzz_jbig2/crash-*` files (May 2026 local fuzzing corpus)
still panic on current HEAD (`5a5f0e24`) with `attempt to add with overflow` at this exact line.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Standalone repro against the real public API (`hayro_jbig2::Image::new`/`decode`), fed one of the 17 stale crash files (`crash-6db743ce82b100c46f63398c7ff127d1cbe18cd8`, 113 bytes) | `cargo run` (dev profile, debug-assertions on by default), 3 consecutive runs | Panics 3/3 runs: `attempt to add with overflow` at `hayro-jbig2/src/huffman_table.rs:455:25`, exact backtrace through `HuffmanTable::decode` -> `decode::symbol::decode` -> `decode_segments` -> `Image::decode`/`decode_with` |
| Sweep of all 17 stale `hayro-fuzz/artifacts/fuzz_jbig2/crash-*` files through the same public entry point | dev profile, pristine tree | 17/17 panic, all at the identical `huffman_table.rs:455` location with the identical message |
| Manual boundary-value construction (no fuzzer needed - see root cause) via a `#[cfg(test)]` unit test added directly in `huffman_table.rs`, calling the private `HuffmanNode::decode_from` on a single synthetic leaf node with `range_length = 32` and a crafted 4-byte offset (`0x7FFFFFFF`) | dev profile | Confirmed both the `+` (add-overflow) and `-` (subtract/underflow) branches panic identically on the unpatched code, and that a normal small offset still decodes correctly (`65808 + 5 = 65813`, `-257 - 5 = -262`) - this is what pinned down the exact triggering condition rather than just "some byte pattern in a 113-byte fuzzer file" |

Only one approach was needed (debug-assertions build + the existing fuzzer-found input, corroborated
by direct boundary-value construction) - the bug class (unchecked arithmetic on an attacker-supplied
value) doesn't need `miri`/ASAN/a fresh fuzz harness; the panic message itself is the full diagnostic.

## Status

**Confirmed**, reproduces **3/3 runs** (dev/debug-assertions build) on the standalone repro, and
**17/17** of the pre-existing stale fuzz corpus files, deterministically (same input -> same panic
every time, no layout/seed dependence).

**Fully reproducible**: **Yes.** The repro (`repro/`) calls `hayro_jbig2::Image::new`/`Image::decode`
- the crate's real, sole public decode entry point - directly on a real, standalone JBIG2 file
(`crash_input.jb2`, one of the 17 stale fuzzer-found crash files). Per `CONFIRMATION.md` section 11,
a malformed file handed to a parser's own public decode entry point *is* the genuine external input;
there is no second participant (peer/caller) being impersonated here, so the single-process,
single-participant repro shape is the correct one for this finding, not a shortcut around it.

## Class

crash (reachable `panic!` via unchecked integer arithmetic on attacker-controlled input; would be a
denial-of-service in any caller that doesn't wrap JBIG2 decoding in `catch_unwind`, e.g. hayro's own
PDF `JBIG2Decode` filter pipeline).

## Root cause

`HuffmanNode::decode_from` (B.4 "Using a Huffman table") ends every successful Huffman-tree walk at
a `Leaf` node. For a non-out-of-band leaf, it reads `leaf.range_length` bits straight off the
bitstream as `range_offset` (`HTOFFSET` in the spec) and then computes:

```rust
let value = if leaf.is_lower {
    leaf.range_low - range_offset
} else {
    leaf.range_low + range_offset
};
```

Every one of the 15 standard tables (`TABLE_A` through `TABLE_O`, in
`hayro-jbig2/src/huffman_tables_generated.rs`) has at least one "unbounded" leaf line with
`range_length: 32` (the encoding's way of saying "this line's value can be arbitrarily large" -
JBIG2 Annex B, table lines for the open-ended top/bottom of a table's range). `read_bits(32)` can
legitimately return any 32-bit pattern, including values like `0x7FFFFFFF`
(2147483647) that, reinterpreted via `as i32`, are already close to `i32::MAX`/`i32::MIN`.
`leaf.range_low` for these unbounded lines is itself a nonzero constant (e.g. `65808` for `TABLE_A`,
`-257` for `TABLE_C`'s lower line) - so `range_low + range_offset` (or `range_low - range_offset`)
overflows/underflows `i32` for a large fraction of the 2^32 possible offset values, with no bounds
check anywhere between reading the bits and doing the arithmetic. A conforming encoder would never
emit such an extreme offset, but nothing in the decoder enforces that - the value is fully
attacker-controlled.

This is the same class of issue prior overflow-hardening commit `3bae23c0` ("hayro-syntax: Apply
some hardening against overflows", #1194) fixed elsewhere in the codebase (`checked_add`/
`checked_mul` on values derived from untrusted input), just not yet applied to this specific
computation in `hayro-jbig2`.

## Introduced by

469f628 "hayro-jbig2: Support reading huffman tables (#728)" (2025-12-23) - `git log -S"range_low"` on `huffman_table.rs` finds this as the commit that introduced the `LeafData`/`range_low +/- range_offset` arithmetic from scratch, with no bounds check from the start.

## The fix

Changed both arithmetic operations to `checked_add`/`checked_sub`, mapping a `None` (overflow) to
`HuffmanError::InvalidCode` - the same error `decode_from` already returns for every other
malformed-encoding case in this function (invalid code, missing child, empty node), so a stream
that tries to encode this out-of-range value is now cleanly rejected as invalid rather than crashing
the process:

```rust
let value = if leaf.is_lower {
    leaf.range_low
        .checked_sub(range_offset)
        .ok_or(HuffmanError::InvalidCode)?
} else {
    leaf.range_low
        .checked_add(range_offset)
        .ok_or(HuffmanError::InvalidCode)?
};
```

Also added a `#[cfg(test)] mod overflow_boundary_tests` in the same file (matching the test style
`3bae23c0` used for its own hardening) covering: the add-overflow branch, the subtract-underflow
branch, and a sanity check that small in-range offsets still decode to the mathematically correct
value on both branches - see "varying the input" below.

Verified against the patched tree:
- The standalone `repro/` (same `crash_input.jb2`) no longer panics: `did not reproduce - decode
  completed without panicking`, 3/3 runs.
- All 17 stale `hayro-fuzz/artifacts/fuzz_jbig2/crash-*` files: `ok=17 panicked=0` (was `ok=0
  panicked=17` before the fix).
- Per `CONFIRMATION.md` section 7 (varying the input within the same bug class, not just the exact
  triggering byte sequence): a boundary-value unit test constructing a synthetic leaf directly
  (bypassing the need to hand-encode a full Huffman prefix code) with `range_length = 32` and an
  offset of `0x7FFFFFFF` confirms **both** the `+` and `-` branches now return
  `Err(HuffmanError::InvalidCode)` instead of panicking, and that ordinary small offsets on the same
  leaf shapes still decode to the correct value (`65808 + 5 = 65813`, `-257 - 5 = -262`) - so the fix
  isn't narrowly tailored to the one 113-byte fuzzer input.

- Project test suite (`cargo test -p hayro-jbig2`):
  - Unit tests (`--lib`): **before** (pristine): 2 passed, 0 failed
    (`segment::tests::test_segment_header_example_1`/`_2`). **After** (patched): 5 passed, 0 failed
    (the same 2 tests + the 3 new `overflow_boundary_tests`).
  - Integration test `--test asset_suite`: fails identically before and after with `No test inputs
    were found. Run \`python sync.py\` to download them.` - a pre-existing environment condition
    (this sandbox has no access to download the external test-asset corpus that test needs),
    unrelated to this change; not counted as a regression.
  - `cargo clippy -p hayro-jbig2 --lib` and `cargo fmt -p hayro-jbig2 -- --check`: clean after the
    fix (patch was formatted with `cargo fmt` before capture).

The working tree was reverted (`git checkout -- hayro-jbig2/src/huffman_table.rs`) immediately after
this verification; the fix lives only in `patch.diff` here now.

## What's not done / open questions

- The same "leaf with `range_length == 32` + fixed `range_low`" shape appears in every one of the 15
  standard tables (`TABLE_A`..`TABLE_O`), not just the one line the 17 stale crash files happen to
  hit - the fix is in the single shared `decode_from` function all of them go through, so all are
  covered by this one change; not independently re-verified crash file by crash file beyond the
  sweep already described.
- Custom (in-stream, non-standard) Huffman tables built via `HuffmanTable::read_custom`/`build` can
  also produce leaf lines with `range_length` up to 32 (bounded only by `read_bits`'s own
  `debug_assert!(count <= 32)` and a `u8` field) - the same fix applies to those too since they go
  through the identical `decode_from` code path; not separately fuzzed as a distinct case this
  session.
- Two other, unrelated JBIG2-fuzzing leads from the same pre-session triage remain open in
  `IDEAS.md`: an apparently-unbounded `Image::decode` hang on huge width/height, and a
  stack-overflow reachable via `fuzz_pdf_render` - neither investigated this session.
