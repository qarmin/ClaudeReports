# imageops::resize (free function) - vertical_sample row/pixel-slice fast path

`src/imageops/sample.rs`'s `vertical_sample` (the vertical convolution pass feeding the standalone
`imageops::resize` free function, `benches/imageops.rs::bench_resize`'s target - a completely
separate, older implementation from `DynamicImage::resize`'s pic-scale-safe backend, per
`IDEAS.md`'s original entry for this row) iterates `image.get_pixel(x, left + i)` /
`out.put_pixel(x, outy, pix)` through the generic `GenericImageView`/`GenericImage` trait methods
for every output pixel and every filter tap. `horizontal_sample` in the same file already has a
direct-slice fast path (it writes straight into `out.subpixels_mut()`); `vertical_sample` did not.
This candidate adds the matching fast path, gated on the same `to_pixel_view()` mechanism used by
candidates 001/004 in `affine.rs`, firing only when the source exposes a canonical packed-pixel
view (`width_stride == pix_stride`), falling back to the original per-pixel loop otherwise.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Benchmark: the crate's own pre-existing
`benches/imageops.rs::bench_resize` ("resize 400x300 <Filter>" cases, 1920x1080 `RgbImage` source),
reused as-is - no bench changes were needed since this candidate's target function was already
reachable from an existing bench, unlike candidates 001/002/004 which needed a new criterion case
added first.

All numbers are from alternating A/B runs of two prebuilt `--release` criterion binaries (baseline
vs. patched), `--warm-up-time 1 --measurement-time 2`, seconds apart on the same machine, per
MEASUREMENT.md's paired-comparison discipline (`uptime` load ~10-11 on 24 cores at the start of
this candidate's measurement, dropping to ~3-8 by the end; within this workspace's established
noise tolerance, see `README.md`).

**`resize 400x300 Triangle`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 14.878 ms | 15.029 ms | 13.964 ms | 14.724 ms | 14.745 ms |
| patched | 10.197 ms | 10.745 ms | 11.145 ms | 10.803 ms | 10.679 ms |

Patched faster in 5/5 pairs. Median baseline 14.745 ms, median patched 10.745 ms (~27% faster).

**`resize 400x300 Lanczos3`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 36.644 ms | 36.830 ms | 35.903 ms | 36.132 ms | 38.808 ms |
| patched | 28.071 ms | 28.197 ms | 28.170 ms | 27.496 ms | 27.757 ms |

Patched faster in 5/5 pairs. Median baseline 36.644 ms, median patched 28.071 ms (~23% faster).

**`resize 400x300 Triangle`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 7.0296 ms | 6.9358 ms | 7.0091 ms | 6.8127 ms | 6.8856 ms |
| patched | 5.2862 ms | 5.2288 ms | 5.1303 ms | 5.1252 ms | 5.1948 ms |

Patched faster in 5/5 pairs. Median baseline 6.9358 ms, median patched 5.1948 ms (~25% faster).

**`resize 400x300 Lanczos3`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 |
|---|---|---|---|
| baseline | 18.373 ms | 18.469 ms | 17.862 ms |
| patched | 13.700 ms | 13.664 ms | 13.650 ms |

Patched faster in 3/3 pairs. Median baseline 18.373 ms, median patched 13.664 ms (~26% faster).

Note: `imageops::resize` is entirely single-threaded (no rayon anywhere in this code path, unlike
candidate 002's `DynamicImage::resize`); "multi-core (unpinned)" here means "not pinned to one
core", not "runs in parallel" - it's included per SKILL.md point 6 as the realistic-wall-clock
check, and shows the same win as the pinned single-core numbers, i.e. no regression from
unpinning.

## Result

A real, consistent win across all 18 paired runs (10 single-core + 8 multi-core across the two
filters tested, 5/5 or 3/3 in every group): roughly 23-27% faster wall-clock for
`imageops::resize` at 1920x1080 -> 400x300, for both a cheap filter (`Triangle`, 2 taps) and an
expensive one (`Lanczos3`, up to ~13 taps at this ratio).

## Does it change behavior/output?

- **Bit-exact?** Yes. `cargo test --release --lib` passes 331/331 both before and after (same
  count, no new failures).
- Additionally (session-only, not part of `patch.diff`): wrote and ran a temporary standalone
  check (`examples/_vsample_check_tmp.rs`, removed before capturing `patch.diff`) comparing
  `imageops::resize` end-to-end (`vertical_sample` -> `horizontal_sample`) between the baseline and
  patched builds, hashing the raw output buffer, for:
  - a 37x29 `RgbImage`, downscaled to 11x9 and upscaled to 53x41, for all 5 `FilterType` variants
    (`Nearest`, `Triangle`, `CatmullRom`, `Gaussian`, `Lanczos3`);
  - a 23x19 `RgbaImage` (4-channel pixel type, exercising the "unused trailing channel stays at
    the pre-existing initial value" replication) downscaled with `Triangle`;
  - a `SubImage` crop (9x6 out of a 19x13 parent `RgbImage`) resized with `Triangle` and
    `Lanczos3` - the crop has `width_stride == pix_stride` (fast path fires) but the row stride
    belongs to the wider parent buffer, exercising the same "packed row, non-contiguous buffer"
    case candidates 001/004 checked;
  - the same-size no-op path (`copy_from`, untouched by this patch).

  All variants produced identical hashes between baseline and patched builds.
- **Is this a breaking change?** No - `imageops::resize`'s public signature, panics, and numeric
  results are unchanged.

## Why the result looks the way it does

Same mechanism as candidates 001/004: the original loop calls `image.get_pixel(x, left + i)`
through the generic `GenericImageView` trait method once per (output pixel, filter tap) pair -
each a full trait-method dispatch including index computation - and `out.put_pixel(x, outy, pix)`
once per output pixel through `GenericImage`. The fast path replaces both with direct slice
indexing into the source's packed-pixel view and the output's raw `f32` sample buffer
(`out.subpixels_mut()`), the same style `horizontal_sample` in this file already used. Unlike
`horizontal_sample`'s access pattern (contiguous within a row, scanned row-by-row), `vertical_sample`
inherently strides across rows for a fixed output column (it iterates filter taps across different
`y` values for the same `x`), so the fast path does not turn this into a fully sequential memory
scan the way candidate 004's row-reversal fast path did - the win here comes purely from removing
the per-access trait-dispatch/bounds-check overhead, not from an improved memory-access pattern,
which likely explains why the win (~23-27%) lands closer to `rotate180_in_place`/
`flip_horizontal_in_place`'s in-place range (candidates 001/004, ~11-27%) than to
`flip_vertical_in`/`flip_horizontal_in`'s out-of-place range (~1.5-9x).

## What's not done / open questions

- `horizontal_sample` in the same file is unaffected by this patch - it already had a direct-slice
  fast path (see `README.md`'s reuse of it as evidence for how to shape this one) and wasn't a
  bottleneck by itself for this candidate; not separately re-measured here.
- Found in passing while validating correctness, **not evaluated as part of this performance
  candidate**: `vertical_sample`'s original per-pixel loop initializes its accumulator to
  `Rgba([1.0; 4])` rather than `[0.0; 4]` before summing the (already-normalized, sum-to-1)
  weighted contributions, which adds a flat `+1.0` bias to every channel of every output pixel
  produced by `imageops::resize` whenever a vertical resize is involved (i.e. essentially always,
  since `resize` always calls `vertical_sample` before `horizontal_sample`). This candidate's fast
  path faithfully reproduces the same `+1.0`-biased initial value for bit-exact parity with the
  original (confirmed identical hashes including this artifact) rather than fixing it - fixing it
  would change `imageops::resize`'s numeric output and is a correctness question, not a
  performance one. Reported separately: `___OTHER___/001.RESIZE_VERTICAL_SAMPLE_OFF_BY_ONE/`.
- Did not profile with `perf`/disassembly to confirm the "removed dispatch overhead, not improved
  memory pattern" explanation above - `perf` remains unavailable in this environment (see
  `README.md`); the explanation is inferred from the measured effect-size band matching the
  in-place candidates rather than the out-of-place ones.
- `rotate90_in`/`rotate270_in` (a different, not-yet-tried entry in `IDEAS.md`'s backlog) and
  `imageops::thumbnail` (also in the backlog, not yet profiled) remain open.
