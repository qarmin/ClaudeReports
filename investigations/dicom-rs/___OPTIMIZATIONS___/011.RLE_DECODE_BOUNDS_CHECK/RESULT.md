# RLE Lossless decode: remove per-element bounds checks in the interleave loop

A fresh discovery this session. Profiling `decode_pixel_data()` on `WG04/RLE/CT1_RLE` (a real
512x512, 16-bit monochrome RLE-Lossless test fixture) with callgrind showed the
`RleLosslessAdapter::decode`'s final interleave step - which scatters each decoded RLE segment's
bytes into their final strided position in the output buffer - spending real time in
`core::slice::index.rs` (bounds-checked indexing) and `core::iter::adapters::step_by` (~11.3%
combined of total instructions), on top of the function's own body (~4.9%). The loop indexed both
the source and destination buffers manually (`dst[base_offset + dst_index] = decoded_segment[decoded_index]`),
which re-checks `dst_index`/`decoded_index` against each buffer's *full* length on every single
element, even though the whole loop's index range is provably in-bounds by construction (a
compile-time-invisible invariant, not something the optimizer can prove from the manual indexing
alone).

## What was tried

Rewrote the innermost interleave loop in `transfer-syntax-registry/src/adapters/rle_lossless.rs`'s
`decode` to slice the destination range once up front (`&mut dst[base_offset + start..base_offset + end]`,
one bounds check) and write through `.iter_mut().step_by(stride).zip(decoded_segment.iter())`
instead of indexing both buffers by computed offsets inside the loop - a safe, idiomatic Rust
rewrite (no `unsafe`) that lets the compiler prove every access in-bounds from the slice's own
length, rather than re-checking against `dst`'s full length on every element.

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s new `decode-raw` mode
(`PixelDecoder::decode_pixel_data()` alone, isolated from any LUT/windowing/colorspace work that
`colorspace`/`tovec` also include) against the real `WG04/RLE/CT1_RLE` fixture (512x512, 16-bit,
RLE Lossless, 262,144 pixels).

| Variant | multi-core p50 | single-core p50 (taskset -c 0) |
|---|---|---|
| baseline (indexed loop) | 0.325/0.341 ms (2 rounds) | 0.336 ms |
| patched (iterator-based) | 0.296/0.305 ms (2 rounds) | 0.300 ms |

## Result

**-9.0% multi-core (9/9, consistent), -10.8% multi-core at `RUNS=15` after a borderline 7/9 second
round prompted a re-check (per this workspace's own established discipline for ambiguous results -
`RUNS=15` came back 15/15, resolving it decisively), -10.5% single-core (`taskset -c 0`, 8/9,
consistent).** A real, solid win - `RleLosslessAdapter::decode` has no rayon dependency at all, so
single-core and multi-core numbers agree closely as expected.

## Does it change behavior/output?

- **Bit-exact?** Yes. `sha256sum` of the fully decoded+converted image (`dump-image`) and the raw
  decoded-pixel-data FNV-1a checksum (`decode-raw`) are identical between baseline and patched
  builds.
- **Is this a breaking change?** No public API/type changes - `RleLosslessAdapter::decode`'s
  signature and behavior are unchanged, only its internal loop structure.
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures. The 2 RLE-specific integration
  tests (`transfer-syntax-registry/tests/rle.rs`, built with `--features rle` since they're gated
  behind that feature) also pass unchanged.

## Why the result looks the way it does

This loop runs once per (frame x sample x byte-offset) combination, iterating `rows * cols` times
each - for this 512x512 single-frame single-sample 16-bit file, 2 byte-offsets x 262,144 pixels =
524,288 total iterations, each previously paying two bounds checks (one per buffer) plus whatever
overhead `step_by` combined with raw index arithmetic added on top. Slicing once and iterating
lets LLVM either eliminate the bounds checks entirely (since each iterator's own length bound is
now locally provable) or at minimum batch/vectorize the check differently - consistent with
HEURISTICS.md's general finding that a scalar, per-element loop doing redundant bounds-checked
indexing is a reliable, moderate (5-15%) win once found in a real profile.

## What's not done / open questions

- Not attempted: extending the same fix to `RleLosslessAdapter::decode_frame` (a near-identical
  per-frame variant of the same function, not profiled or measured separately this session - very
  likely benefits the same way, given it shares the same interleave loop shape, but not confirmed).
- Not investigated: whether the `step_by`-based destination iteration itself (as opposed to the
  bounds-check removal) could be restructured further (e.g. processing multiple interleave
  "lanes" - one per byte-offset/sample combination - together in a single pass over
  `decoded_segment` instead of one pass per lane) for a larger win - this candidate only removed
  the redundant bounds checks, not the loop's fundamental structure.
- The other two hot spots from the same profile (`Lut<u16>::new_rescale_and_normalize`/
  `new_with_fn`'s LUT-table-construction f64 math, ~15.6% combined) were noticed but not
  investigated this candidate - added to `IDEAS.md` as a separate, real lead.
