# imageops::thumbnail - thumbnail_sample_block get_pixel fast path

`src/imageops/sample.rs`'s `thumbnail` (the fast-integer algorithm backing
`imageops::thumbnail`/`DynamicImage::thumbnail`/`thumbnail_exact`, a completely separate,
single-threaded implementation from `DynamicImage::resize`'s pic-scale-safe backend and from the
standalone `imageops::resize` free function - per `IDEAS.md`'s original entry for this row)
dispatches almost every output pixel to `thumbnail_sample_block`, which sums a rectangular window
of source pixels via `image.get_pixel(x, y)` through the generic `GenericImageView` trait method,
once per source pixel in the window. This is the dominant cost for the common "big photo -> small
icon/preview" use case, since a large downscale ratio means `bottom != top && left != right` for
essentially every output pixel (the block case), while the three fraction-based helper functions
(`thumbnail_sample_fraction_horizontal/vertical/both`, only reachable when scaling up in a
dimension or landing exactly on a boundary) are only ever called `O(width + height)` times, not
`O(width * height)`, and were left unchanged in this candidate.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Benchmark: a new criterion case,
`thumbnail 4032x3024 -> 200x150`, added to the crate's own `benches/imageops.rs` (bundled into
this candidate's `patch.diff`, following the convention established by candidates 001/002/004 -
no prior bench in this repo exercised `imageops::thumbnail` at all). Source: a synthetic
4032x3024 `RgbImage`, matching the size used elsewhere in this workspace's callgrind profiling
(`IDEAS.md`'s JPEG/Lanczos3 entries), downscaled to 200x150 - the exact scenario the `IDEAS.md`
entry for this candidate suggested as the representative case.

All numbers are from alternating A/B runs of two prebuilt `--release` criterion binaries
(baseline vs. patched, both built with the bench change present so only the source patch under
test differs), `--warm-up-time 1 --measurement-time 2`, using this workspace's `tmp/run_bench.sh`
helper. `uptime` load average was ~3.6-10.4 on 24 logical cores across the measurement window
(within this workspace's established noise tolerance, see `README.md`).

**`thumbnail 4032x3024 -> 200x150`, single-core (`taskset -c 3`):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 13.018 ms | 17.253 ms | 16.734 ms | 18.298 ms | 17.822 ms |
| patched | 5.783 ms | 6.905 ms | 7.047 ms | 7.396 ms | 7.022 ms |

Patched faster in 5/5 pairs. Median baseline 17.253 ms, median patched 7.022 ms (~2.46x faster).

**`thumbnail 4032x3024 -> 200x150`, multi-core (unpinned):**

| Variant | run 1 | run 2 | run 3 | run 4 | run 5 |
|---|---|---|---|---|---|
| baseline | 17.612 ms | 17.748 ms | 16.534 ms | 17.179 ms | 16.904 ms |
| patched | 7.162 ms | 6.994 ms | 7.138 ms | 7.227 ms | 7.258 ms |

Patched faster in 5/5 pairs. Median baseline 17.179 ms, median patched 7.162 ms (~2.40x faster).

Note: `imageops::thumbnail` is entirely single-threaded (no rayon anywhere in this code path,
same as candidate 005's `imageops::resize`); "multi-core (unpinned)" means "not pinned to one
core", included per SKILL.md point 6 as the realistic-wall-clock check - it shows the same win as
the pinned single-core numbers, i.e. no regression from unpinning.

## Result

A real, consistent win across all 10 paired runs (5 single-core + 5 multi-core, 5/5 in each
group): roughly 2.4-2.5x faster wall-clock (baseline ~17ms median, patched ~7ms median) for
`imageops::thumbnail` at 4032x3024 -> 200x150, a representative "large photo -> small
icon/preview" downscale.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cargo test --release --lib` passes 331/331 both before and after (same
  count, no new failures). `cargo clippy --release --lib --benches` and `rustfmt --check` on both
  touched files report no warnings/diffs.
- Additionally (session-only, not part of `patch.diff`): wrote and ran a temporary standalone
  check (`examples/_thumbnail_check_tmp.rs`, removed before capturing `patch.diff`) comparing
  `imageops::thumbnail` end-to-end between the baseline and patched builds, hashing the raw
  output buffer, for:
  - a 4032x3024 `RgbImage` downscaled to 200x150, 1x1, and 63x41 (block case dominant), and
    "resized" to its own 4032x3024 (identity, still goes through the block path with 1x1 windows);
  - a 577x401 `RgbaImage` (4-channel pixel type) downscaled to 37x29 and upscaled to 600x500
    (exercises the fraction-based helpers, unaffected by this patch, as a control);
  - a 300x200 `Luma<u8>` (1-channel) and a 300x200 `Luma<u16>` (16-bit subpixel) image, both
    downscaled to 40x30;
  - a `SubImage` crop (211x133 out of a 400x300 parent `RgbImage`) downscaled to 29x17 - the crop
    has `width_stride == pix_stride` (fast path fires) but the row stride belongs to the wider
    parent buffer, exercising the same "packed row, non-contiguous buffer" case candidates
    001/004/005 checked;
  - `DynamicImage::thumbnail_exact` and `DynamicImage::thumbnail` (the aspect-ratio-preserving
    public wrapper), end to end on the 4032x3024 source.

  All variants produced identical hashes between baseline and patched builds.
- **Is this a breaking change?** No - `imageops::thumbnail`'s public signature, panics, and
  numeric results are unchanged.

## Why the result looks the way it does

Same mechanism as candidates 001/004/005: `thumbnail_sample_block`'s "standard case" inner loop
called `image.get_pixel(x, y)` through the generic `GenericImageView` trait method once per
(output pixel, source pixel in its window) pair, each a full trait-method dispatch plus bounds
check. The window size itself scales with the downscale ratio (e.g. for a 4032x3024 -> 200x150
resize, each output pixel averages roughly a 20x20 = 400-pixel window), so this loop's per-access
overhead is paid `new_width * new_height * window_area` times, i.e. once per *source* pixel, not
once per output pixel - this is why the win here (~2.4-2.5x) lands closer to `flip_vertical_in`/
`flip_horizontal_in`'s out-of-place range (candidates 001/004, ~1.5-9x) than to
`vertical_sample`'s ~23-27% (candidate 005), even though the underlying fix (direct slice
indexing via `to_pixel_view()`, gated on `width_stride == pix_stride`) is identical: removing the
per-source-pixel dispatch overhead matters more here because there are many more source-pixel
accesses per output pixel than in a filtered resize's fixed-tap-count convolution.

The patch only touches the "standard case" branch inside `thumbnail_sample_block` (where
`pixel_count <= 256*256`); the pre-existing huge-block-splitting recursion (for extreme downscale
ratios producing >256*256-pixel windows) still recurses into `thumbnail_sample_block` itself,
which re-checks the same `fast_view` (computed once in `thumbnail()` and threaded through, not
recomputed per call) - so the fast path still fires for each split sub-block, just not
specifically exercised as its own measured case in this candidate (256*256 = 65536 pixels per
output pixel is an extreme ratio, not the representative scenario benchmarked here).

## What's not done / open questions

- `thumbnail_sample_fraction_horizontal`, `thumbnail_sample_fraction_vertical`, and
  `thumbnail_sample_fraction_both` (the three helpers used only when scaling up in a dimension,
  or for the boundary output pixel of a dimension) still use `image.get_pixel`/generic dispatch
  unchanged. They are called `O(width + height)` times total, not `O(width * height)`, so they are
  not the bottleneck for the representative downscale scenario this candidate measured - not
  converted here to keep this candidate scoped to the one dominant hot loop. Worth a follow-up if
  a future workload calls `thumbnail` primarily for upscaling.
- Did not profile with `perf`/disassembly to confirm the "more source-pixel accesses per output
  pixel here than in candidate 005's resize" explanation above precisely (e.g. actual window size
  distribution) - `perf` remains unavailable in this environment (see `README.md`); the
  explanation follows directly from the algorithm's window-size arithmetic rather than needing a
  profiler to confirm.
- Only RGB8/RGBA8/Luma8/Luma16 pixel types and one `SubImage` case were hash-verified; other
  `Pixel` implementations (e.g. floating-point subpixel types) were not separately checked, though
  the fast path's logic does not depend on subpixel type beyond what `ThumbnailSum`/`Enlargeable`
  already require.
