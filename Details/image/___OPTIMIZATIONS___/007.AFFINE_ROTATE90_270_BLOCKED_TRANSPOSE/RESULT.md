# rotate90_in / rotate270_in: blocked-transpose fast path

`src/imageops/affine.rs`'s `rotate90_in` and `rotate270_in` iterate every pixel via the generic
`GenericImageView::get_pixel`/`GenericImage::put_pixel` trait methods (same family as the
already-converted `flip_vertical_in`/`rotate180_in_place`/`flip_horizontal_in`/
`flip_horizontal_in_place` in `SUMMARY.md` #001/#004), but these two transpose x<->y instead of
doing a row/linear reversal, so the naive row-major read / column-major write pattern is
cache-unfriendly for tall images even after removing trait dispatch. Added a fast path (gated on
the same existing `to_pixel_view()` mechanism, firing when `width_stride == pix_stride`) that
walks the image in 64x64-pixel tiles: within one tile the destination-side access stays confined
to 64 rows x 64 pixels, so it remains cache-resident, unlike an unblocked transpose whose write
side touches a new destination row (`dst_row_len` apart) for every single source pixel.

## What was tried

Base commit: `44ce9226c541dd2b11897b9ef07156a28871cdb7`. Benchmark: `benches/imageops.rs`'s
existing `rotate90`/`rotate270` criterion cases (exercise `rotate90_in`/`rotate270_in` via the
public wrappers, 1920x1080 `RgbImage`) - no bench file changes needed. `rotate180` (unmodified
code path) was also measured each session as a sanity check that any apparent win wasn't just
system-noise/binary-identity drift.

All numbers are from alternating A/B runs of two prebuilt `--release` criterion binaries
(baseline vs. patched), `--warm-up-time 1 --measurement-time 2`, seconds apart on the same
machine (this machine's load varied ~2.5-8.8 across the session; see `README.md`'s note on why
single-binary repeated runs aren't trusted here).

**`rotate90`, single-core (`taskset -c 3`), ms:**

| run | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|---|---|---|---|---|---|---|---|---|---|
| baseline | 1.194 | 1.219 | 1.260 | 1.205 | 1.227 | 1.185 | 2.922 | 2.741 | 3.158 |
| patched | 1.054 | 1.084 | 1.085 | 1.116 | 1.088 | 1.046 | 2.155 | 2.172 | 2.387 |

Patched faster in 9/9 pairs. Median baseline 1.227 ms, median patched 1.088 ms (~11.3% faster).
(runs 7-9 landed in a higher-system-load window than 1-6; the ratio holds either way.)

**`rotate90`, multi-core (unpinned), ms:**

| run | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| baseline | 1.206 | 1.224 | 1.200 | 1.252 |
| patched | 1.053 | 1.081 | 1.081 | 1.095 |

Patched faster in 4/4 pairs. Median baseline 1.215 ms, median patched 1.081 ms (~11.0% faster).

**`rotate270`, single-core (`taskset -c 3`), ms:**

| run | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| baseline | 1.783 | 1.822 | 2.075 | 1.826 | 1.786 | 1.845 | 3.597 | 3.623 |
| patched | 1.050 | 1.086 | 1.059 | 1.166 | 1.127 | 2.531 | 2.291 | 2.224 |

Patched faster in 7/8 pairs (run 6 is the one exception - a noise spike in the patched side, not
reproduced in any neighboring run). Median baseline 1.835 ms, median patched 1.147 ms (~37.5%
faster).

**`rotate270`, multi-core (unpinned), ms:**

| run | 1 | 2 | 3 | 4 |
|---|---|---|---|---|
| baseline | 1.806 | 1.855 | 1.854 | 1.868 |
| patched | 1.079 | 1.084 | 1.099 | 1.065 |

Patched faster in 4/4 pairs. Median baseline 1.855 ms, median patched 1.082 ms (~41.7% faster).

**`rotate180` (unmodified code path), single-core, ms - sanity check:**

| run | 1 | 2 | 3 |
|---|---|---|---|
| baseline | 3.164 | 3.269 | 3.325 |
| patched | 3.343 | 3.258 | 3.314 |

No consistent direction (2/3 pairs favor baseline, by <3%), confirming the rotate90/270 win isn't
an artifact of comparing two binaries that differ for unrelated reasons.

## Result

Both `rotate90_in` and `rotate270_in` are faster with the blocked-transpose fast path, on both
single-core and unpinned multi-core measurements, with consistent pair-win majorities (9/9, 4/4,
7/8, 4/4). `rotate270_in` shows a substantially larger win (~37-42%) than `rotate90_in` (~11%)
despite both using the same tiling shape and tile size - see "Why the result looks the way it
does" below for the likely reason.

## Does it change behavior/output?

- **Bit-exact?** Yes. `cargo test --release` (all suites) passes 550/551 both before and after
  the patch (identical count and identical single failure both times -
  `tests/regression.rs::resizing_with_catmul`, a pre-existing failure unrelated to this patch and
  to affine.rs - confirmed by running the full suite against the unpatched tree via `git stash`
  before writing this file). No new failures, no fewer passes.
- Additionally (session-only, not part of `patch.diff`): wrote and ran a temporary standalone
  check (`examples/_rotate_check.rs`, removed before capturing `patch.diff`) comparing
  `rotate90`/`rotate270` against a reference per-pixel implementation across 11 `Rgb<u8>` sizes
  chosen to exercise every edge of the 64x64 blocked-transpose loop (1x1, 2x3, 37x29, 63x65,
  64x64, 65x63, 128x128, 129x127, 200x130, 300x5, 5x300), plus `Rgba<u8>` and `Luma<u16>` at
  129x127, plus a `SubImage` crop (9x6 out of a 19x13 parent) exercising the packed-row-but-
  non-contiguous-buffer case the fast path's `to_pixel_view()` gate has to handle correctly. All
  14 checks matched the reference exactly.
- **Is this a breaking change?** No - `imageops::affine`'s public functions keep their existing
  signatures, panics, and numeric results.

## Why the result looks the way it does

Same trait-dispatch-removal mechanism as candidates 001/004/005/006, plus the blocking this
candidate's `IDEAS.md` entry called for: for a fixed source row `y`, the destination write
address for `rotate90_in` is `x * dst_row_len + (h0-1-y) * pix_stride` - as `x` varies within one
64-wide tile, that only touches 64 different destination rows, each written at the same column
offset, which is 64 * pix_stride bytes of cache footprint per row touched, not a full row-length
stride jump across the whole buffer on every pixel like the unblocked version would.

`rotate270_in`'s larger win is likely because its dimension mapping happens to line up better with
row-major source scanning for whichever cache/prefetch effect matters most on this machine
(`dst_col = y` is fixed for the whole source row here, only `dst_row = w0-1-x` varies - the
opposite axis from `rotate90_in`, where `dst_col` is the one that varies with `y`) - not
independently confirmed via `perf`/disassembly (`perf` is not installed in this environment, see
`README.md`), so this is an inference from the measured numbers only, consistent with (but not
proof of) a genuine access-pattern asymmetry between the two functions.

## What's not done / open questions

- Did not sweep the `BLOCK = 64` tile size against other values (32/128/256) - 64 was chosen by
  analogy to a typical L1-cache-friendly tile without measuring alternatives; a smaller or larger
  block might close more of the remaining gap (rotate90_in's ~11% is much smaller than
  rotate270_in's ~37-42%, worth checking whether a different block size narrows that gap).
- Did not investigate the `rotate90_in`/`rotate270_in` asymmetry with `perf`/disassembly - `perf`
  is not installed in this environment; the explanation above is a plausible reading of the
  numbers, not independently confirmed.
- The two functions still share almost identical code (block-loop structure duplicated with only
  the index-mapping formula differing) - could be factored into one shared helper parameterized
  by the two index-mapping closures, but wasn't attempted here to keep this patch minimal and
  directly comparable to the unpatched per-function loops.
