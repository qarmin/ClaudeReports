# Lut::new_with_fn: pixel-count threshold + core guard before rayon table construction

A fresh discovery this session, noticed while profiling candidate 011's RLE decode fixture.
`Lut::new_with_fn` (`pixeldata/src/lut.rs`) - the single low-level constructor shared by every
`Lut` creation function (`new_rescale`, `new_rescale_and_window`, `new_rescale_and_lut`, etc.) -
unconditionally dispatches table construction to rayon's `into_par_iter()` whenever the `rayon`
feature is compiled in, regardless of the table size. Table size is `2^bits_stored`: for an 8-bit
image that's only 256 entries, for 16-bit it's 65,536 - a large gap, and unlike
`convert_colorspace_u8`/`u16` (candidate 003) or the LUT-consumer call sites (candidate 006), this
threshold question had never been checked for the table-*construction* step itself.

## What was tried

Extracted the per-entry closure into a `map_entry` binding (needed so the same closure can feed
either a sequential `Iterator` or `rayon::iter::ParallelIterator`, matching the pattern already
used in this workspace's other threshold-guarded functions - `ParallelIterator`/`Iterator` are
different types that can't share one `let iter = ...` binding). Added the same
`rayon_worth_it()`/threshold-guard pair as candidates 003/004/006
(`LUT_RAYON_SIZE_THRESHOLD = 16_384`, reusing the same order-of-magnitude value, since an 8-bit
table (256) is always far below it and a 16-bit table (65,536) is always far above it - this fix
only matters for the boundary in between, which no real `bits_stored` value lands exactly on, but
sets the right behavior if a future caller ever constructs a LUT for an unusual bit depth).

Bench: `___OPTIMIZATIONS___/harness/bench_pipeline`'s `tovec` mode
(`decode_pixel_data().to_vec_frame::<f32>(0)`, which builds a LUT via `Lut::new_rescale_and_window`/
similar and then maps every pixel through it) against `pydicom/vlut_04.dcm` (512x512, 8-bit,
256-entry LUT table, below threshold) and `WG04/RLE/CT1_RLE` (512x512, 16-bit, 65,536-entry table,
above threshold - reused from candidate 011).

| Variant | 8-bit (256-entry table) p50 | 16-bit (65,536-entry table) p50 | single-core, 8-bit, no-rayon build |
|---|---|---|---|
| baseline (unconditional rayon) | 0.149/0.152 ms (2 rounds) | 0.965 ms | 0.120 ms |
| patched (threshold+guard) | 0.094/0.095 ms (2 rounds) | 0.965 ms (no change) | 0.120 ms (no change) |

## Result

**Below the threshold (8-bit, 256-entry table): -37.2%/-37.8% across two independent rounds, 9/9
paired wins both times, consistent** - a large, real win, bigger in relative terms than any of this
workspace's other rayon-threshold candidates (003/004/006), because `to_vec_frame`'s total
per-call cost for this fixture is itself small enough that rayon's fixed per-call dispatch
overhead (spawning/joining a work-stealing task, even for a 256-element range) is a substantial
fraction of it - not just of the table construction step in isolation. **Above the threshold
(16-bit, 65,536-entry table): no consistent change** (-0.0%, 5/9 - not consistent), exactly as
expected since that code path is identical to baseline (same `par_chunks`/`into_par_iter`
dispatch). **Single-core (`--no-default-features --features image,native` build, no rayon linked):
no consistent change** (+0.1%, 3/9 - not consistent), also as expected since the entire
threshold/guard block is behind `#[cfg(feature = "rayon")]` and compiles out completely without
that feature.

## Does it change behavior/output?

- **Bit-exact?** Yes, in both configurations tested - `sha256sum` of the decoded+converted output
  (`dump-image`) is identical between baseline and patched builds on both fixtures. Only the
  dispatch mechanism changed, never the arithmetic.
- **Is this a breaking change?** No public API/type changes. `LUT_RAYON_SIZE_THRESHOLD` and
  `rayon_worth_it()` are private items in `pixeldata/src/lut.rs` (a separate, textually-identical
  duplicate of the same-named helper in candidates 003/004/006/009's own files - each candidate's
  patch is self-contained and independently applicable, so this is intentional duplication per
  this workspace's "one fix per candidate" discipline, not a bug).
- Project test suite: **537 passed / 0 failed / 6 ignored, both before and after** (full
  `cargo test --workspace`, including doctests) - no new failures.

## Why the result looks the way it does

Because `Lut::new_with_fn` is the single shared low-level constructor behind every `Lut` creation
path, this one fix improves every LUT-consuming caller for small-bit-depth images at once
(`to_vec_frame`, `to_dynamic_image`'s monochrome path, etc.) - unlike candidate 006's per-call-site
guards on the *consumer* side (`convert_pixel_slice`'s `map_iter`/`map_par_iter` calls), which
needed one guard per site and hit a real regression on one of them. The larger relative win here
(-37% vs. candidate 003's -32% for a conceptually similar fix) is consistent with 8-bit LUT
construction being an even smaller fixed amount of real work (256 entries) relative to rayon's
fixed dispatch cost than `convert_colorspace_u8`'s below-threshold case was.

## What's not done / open questions

- The exact crossover point between 256 and 65,536 entries was not measured (unlike candidate
  003's own crossover-hunting exercise) - not necessary here since no real DICOM `bits_stored`
  value falls in that range (8, 10, 12, 16 are the practically-occurring values, all far from the
  16,384 threshold either way), but worth knowing if a future caller ever constructs a `Lut` for
  an unusual bit depth close to the threshold.
- Not measured: whether the same guard is worth adding to `Lut::new_rescale_and_normalize`'s own
  min/max-scanning fold over `samples` (a separate, unguarded sequential-only operation in the
  current code, not parallelized at all) - out of scope for this candidate, which only touched
  `new_with_fn`'s table-construction dispatch.
