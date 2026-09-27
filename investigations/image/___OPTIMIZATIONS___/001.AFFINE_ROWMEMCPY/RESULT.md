# flip_vertical_in / rotate180_in_place: row/pixel-slice fast path

`src/imageops/affine.rs`'s `flip_vertical_in` and `rotate180_in_place` iterate every pixel via
the generic `GenericImageView::get_pixel`/`GenericImage::put_pixel` trait methods, even though
both operations are pure data movement (row-order reversal, and a full linear pixel-sequence
reversal respectively) for the common contiguous `ImageBuffer<P, Vec<S>>` case. Added a fast path
to each, gated on the crate's own existing `to_pixel_view()`/`to_pixel_view_mut()` mechanism
(`GenericImageView`/`GenericImage`, already used by `copy_from`/`copy_from_samples` for the same
purpose) so it only fires when the buffer is provably packed/contiguous, falling back to the
original per-pixel loop otherwise.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Benchmark: `benches/imageops.rs`'s
`flip_vertical` case (exercises `flip_vertical_in` via the public `flip_vertical()` wrapper,
1920x1080 `RgbImage`) plus a new `rotate180_in_place` case added to the same file (see
`patch.diff`; no prior bench exercised `rotate180_in_place` directly). All numbers below are from
alternating A/B runs of two prebuilt `--release` criterion binaries (baseline vs. patched) on the
same machine seconds apart, per MEASUREMENT.md's paired-comparison discipline - plain repeated
runs of one binary at a time showed much larger run-to-run scatter (system load varied 3.7-18 over
the session) and were not trusted on their own.

**`flip_vertical` (-> `flip_vertical_in`), single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 2.597 ms | 2.717 ms | 2.612 ms |
| patched | 338 µs | 357 µs | 349 µs |

Patched faster in 3/3 pairs. ~7.5-8x.

**`flip_vertical`, multi-core (unpinned):**

| Variant | run 1 | run 2 |
|---|---|---|
| baseline | 1.301 ms | 1.374 ms |
| patched | 152 µs | 153 µs |

Patched faster in 2/2 pairs. ~8.5-9x.

**`rotate180_in_place`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 1.934 ms | 2.060 ms | 2.103 ms | 1.965 ms | 2.160 ms |
| patched | 1.624 ms | 1.624 ms | 1.617 ms | 1.573 ms | 1.095 ms |

Patched faster in 5/5 pairs. Median baseline ~2.06 ms, median patched ~1.62 ms (~21% faster);
one patched run (1.095 ms) is a clear outlier on the fast side, consistent with occasional
favorable scheduling/turbo rather than a different code path (the fast-path guard is
all-or-nothing per call, not partial).

**`rotate180_in_place`, multi-core (unpinned):**

| Variant | run 1 | run 2 |
|---|---|---|
| baseline | 1.033 ms | 1.265 ms |
| patched | 0.922 ms | 0.918 ms |

Patched faster in 2/2 pairs, ~11-27% faster.

An earlier attempt at "baseline" numbers (recorded, then discarded) accidentally measured the
patched binary twice because the source edit was made before any baseline was captured - see
"Why the result looks the way it does" below. All numbers in the tables above are from a
deliberately re-established true baseline (`git checkout -- src/imageops/affine.rs`, rebuilt)
compared against the patched build via the alternating-binaries method.

## Result

`flip_vertical_in`'s fast path is a large, robust win: ~7.5-9x faster, consistent across every
paired run, single- and multi-core. `rotate180_in_place`'s fast path is a real but modest win:
~11-27% faster, consistent in direction across all 7 paired runs (5 single-core + 2 multi-core)
but far smaller than `flip_vertical_in`'s - see below for why.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cargo test --lib` passes 331/331 both before and after (same count, no new
  failures) on this working tree, including the existing `test_flip_vertical`,
  `test_flip_vertical_in_place`, and `test_rotate180_in_place` unit tests in `affine.rs` itself.
- Additionally (session-only, not part of `patch.diff` - this project's `SKILL.md` doesn't add
  permanent unit tests): wrote and ran a temporary test exercising both functions through a
  `SubImage` crop (`big.view(rect)` / `big.sub_image(rect)`, a 4x5 crop out of a 9x7 buffer). This
  confirmed (a) `flip_vertical_in`'s fast path correctly fires even when the *row* is packed but
  the *view* as a whole isn't (a crop's `width_stride == pix_stride` but `height_stride` still
  belongs to the parent's wider row), producing correct output via the per-row `sh`-strided copy,
  and (b) `rotate180_in_place`'s fast-path guard correctly rejects that same crop (its
  `height_stride` doesn't equal `crop_width * pix_stride`) and falls back to the original
  per-pixel loop, again producing correct output, with pixels outside the crop in the parent
  buffer left untouched. This was removed before capturing `patch.diff`.
- **Is this a breaking change?** No - `imageops::affine`'s public functions keep their existing
  signatures, panics, and (per the tests above) numeric results.

## Why the result looks the way it does

`flip_vertical_in`'s original loop calls `image.get_pixel(x, y)` then
`destination.put_pixel(x, h0-1-y, p)` once per pixel (2.07M pixel visits for the 1920x1080 test
image) - two full trait-method calls including index computation and Option-based bounds
checking. The fast path replaces this with one `copy_from_slice` per row (1080 calls total),
which is why the win is so large.

`rotate180_in_place` was already only about 20% slower than the patched version despite an
outwardly similar per-pixel double loop (`image.get_pixel`/`image.put_pixel`, ~1.04M pixel-pair
visits). Both the original and the patched version end up doing the same total memory traffic (one
full read+write pass over the ~6.2 MB buffer), and because this loop operates on a single
concrete, monomorphized `I: GenericImage` type with both reads and writes going through the same
object, LLVM appears to already optimize the scalar loop close to memcpy-speed for this case -
unlike `flip_vertical_in`, where `image`/`destination` are two independently generic type
parameters. The row/pixel-slice fast path still avoids the per-pixel index/bounds-check overhead,
so the ~11-27% win is real, just much smaller than intuition from "get_pixel/put_pixel in a loop"
alone would suggest - the measured numbers are the actual evidence, not the pattern-matching
heuristic that flagged the candidate.

## What's not done / open questions

- `flip_horizontal_in`/`flip_horizontal_in_place` and `rotate90_in`/`rotate270_in` have the same
  per-pixel pattern (noted in `IDEAS.md`) but were not touched here - `rotate90`/`rotate270`
  transpose the buffer, which doesn't reduce to a simple row or linear-array reversal the same
  way, and would need a different fast-path shape (e.g. blocked transpose) to get a comparable
  win; that's a separate candidate.
- The bench addition (`rotate180_in_place` case in `benches/imageops.rs`) is bundled into
  `patch.diff` alongside the real fix (see `README.md`'s "Instrumentation added" section) since it
  was needed to reach the code at all; it is not itself a performance change.
- Did not investigate why `rotate180_in_place`'s baseline is already close to memcpy-speed in
  more depth (e.g. via `perf` or disassembly) - `perf` is not installed in this environment (see
  `README.md`'s Environment section); the LLVM-optimization explanation above is a plausible
  reading of the numbers, not independently confirmed via disassembly.
