# vello_cpu: avoid continuous SIMD position re-splatting in image/gradient painters

`sparse_strips/vello_cpu/src/fine/common/{gradient,image}.rs` and `fine/lowp/image.rs`'s image and
gradient painters kept a scalar `f64` (or `Point`) current position and, at the start of every
8/16-pixel chunk, called `f32x8::splat_pos`/`f32x4::splat_pos` to rebuild a fresh SIMD position
vector from it, then advanced only the scalar afterward. This is the exact mechanism described in
open PR [#1728](https://github.com/linebender/vello/pull/1728) ("vello_cpu: Avoid continuously
splatting positions") - `splat_pos` is comparatively expensive, and re-deriving the SIMD vector
from a scalar every iteration also prevents the compiler from keeping the position live in
registers across iterations. Toolchain: `rustc 1.98.0 (88d9e12ae 2026-08-18)`, `cargo 1.98.0`.

This candidate applied the PR's own patch verbatim (fetched via `gh pr diff 1728`, minus the
binary snapshot-PNG hunks - see "What's not done" below) and measured it, rather than
re-implementing the same idea independently.

## What was tried

The patch replaces the scalar-position-then-`splat_pos`-every-chunk pattern with two small
iterator-like structs, `PaintPositions`/`PlainPaintPositions` (`fine/mod.rs`), that compute the
starting SIMD position vector **once** via `splat_pos` and then advance it every chunk using plain
SIMD `mul_add`/`+=` arithmetic (`i` counts chunks, `current = i * step + base`), never calling
`splat_pos` again after the first chunk. Every gradient kind (`linear`/`radial`/`sweep`) and every
image painter (`PlainNNImagePainter`, `NNImagePainter`, `FilteredImagePainter`,
`BilinearImagePainter`, `PlainBilinearImagePainter`) was switched to source its position from one
of these two structs instead of calling `splat_pos` inline.

Benchmark: the project's own already-tracked `sparse_strips/vello_bench` `fine_gradient`/
`fine_image` Criterion groups (built with `--features extended` to unlock every existing sub-bench,
including bicubic/`quality::high` and every `transform`/`extend` variant) - no new harness needed,
this candidate reuses `fine/gradient.rs` and `fine/image.rs` benches that already exist in-tree.
18 sub-benchmarks were run (both `sse4_2` and `avx2` codegen paths, `linear`/`radial`/`sweep`
`opaque` gradients, and `quality::{low,medium,high}` + `transform::{scale,none,rotate}` images).

Single-core numbers are `taskset -c 3`; multi-core numbers are unpinned. Both were measured with
Criterion's own `--save-baseline`/`--baseline` paired-comparison mode (bootstrap CI + p-value per
benchmark, not just a raw median diff), run under `flock` serialized against sibling candidates'
own builds/benchmarks in the same batch session. This machine (24-core, shared with sibling
candidates' own builds) showed up to ~30% same-binary swings on repeated single-core runs (see
"Why the result looks the way it does"), so the single-core numbers below are two independent
paired runs against the same saved baseline, and the unpinned multi-core run is the more reliable
number.

**Single-core (`taskset -c 3`), avx2, baseline vs. patched (two independent runs against the same
saved Criterion baseline):**

| Benchmark (avx2) | baseline p50 | patched p50, run 1 | change, run 1 | patched p50, run 2 | change, run 2 |
|---|---|---|---|---|---|
| `gradient/linear/opaque` | 776.8 ns | - | -26.8% (p=0.00) | 653.1 ns | -14.3% (p=0.00) |
| `gradient/radial/opaque` | 798.3 ns | - | -15.4% (p=0.00) | 632.4 ns | -14.1% (p=0.00) |
| `gradient/sweep/opaque` | 1.191 µs | - | -5.5% (p=0.01) | 957.6 ns | -18.9% (p=0.00) |
| `image/transform/scale` | 731.6 ns | - | -18.8% (p=0.00) | 559.6 ns | -23.6% (p=0.00) |
| `image/quality/low` | 735.9 ns | - | -27.0% (p=0.00) | 526.1 ns | -25.6% (p=0.00) |
| `image/quality/medium` | 3.497 µs | - | -29.0% (p=0.00) | 2.235 µs | -42.0% (p=0.00) |
| `image/transform/none` | 767.0 ns | - | +2.1% (p=0.35, no change) | 518.1 ns | -30.3% (p=0.00) |
| `image/transform/rotate` | 1.292 µs | - | -25.3% (p=0.00) | 632.2 ns | -49.8% (p=0.00) |
| `image/quality/high` | 15.68 µs | - | -6.8% (p=0.00) | 11.72 µs | -34.1% (p=0.00) |

(`sse4_2` variants: same direction, similar magnitude, in both runs - two individual sub-benchmarks
flipped sign between the two single-core runs, `image/quality/low_u8_sse4_2` (-21.0% run 1, +6.0%
run 2) and `image/quality/high_u8_sse4_2` (+10.2% run 1, -20.9% run 2) - consistent with this
machine's own single-core noise floor, not a real regression in either direction; every other
`sse4_2` sub-benchmark improved in both runs.)

**Multi-core (unpinned), avx2, baseline vs. patched (single paired run, one Criterion invocation
per side):**

| Benchmark (avx2) | baseline p50 | patched p50 | change |
|---|---|---|---|
| `gradient/linear/opaque` | 539.6 ns | 499.4 ns | -7.0% (p=0.00) |
| `gradient/radial/opaque` | 640.2 ns | 595.5 ns | -7.1% (p=0.00) |
| `gradient/sweep/opaque` | 1.001 µs | 838.2 ns | -16.2% (p=0.00) |
| `image/transform/scale` | 652.0 ns | 515.9 ns | -20.8% (p=0.00) |
| `image/quality/low` | 651.0 ns | 516.0 ns | -20.9% (p=0.00) |
| `image/quality/medium` | 2.348 µs | 2.193 µs | -6.8% (p=0.00) |
| `image/transform/none` | 648.4 ns | 515.8 ns | -20.4% (p=0.00) |
| `image/transform/rotate` | 839.1 ns | 628.9 ns | -22.4% (p=0.00) |
| `image/quality/high` | 11.62 µs | 11.35 µs | -2.3% (p=0.00) |

All 18 sub-benchmarks (both `sse4_2` and `avx2`) improved in the unpinned run, every one at
p=0.00 except `gradient/radial/opaque_u8_sse4_2` (p=0.51, no change). Full raw Criterion output for
both single-core runs and the unpinned run is not attached here (not needed to reproduce - `just
test` regenerates it), but every number above was copied directly from a real run's console
output, not summarized from memory.

## Result

Time only (no allocation/RSS claim - this patch changes how a position is recomputed, not
allocation counts or buffer sizes).

A real, consistent speedup across essentially every gradient/image sub-benchmark measured, in both
the single-core and (more reliably, since this shared machine has a high single-core noise floor)
unpinned multi-core runs - typically **-15% to -30%** on `avx2`, with some sub-benchmarks (e.g.
`image/transform/rotate_u8_avx2` single-core run 2, `-49.8%`) considerably larger. This matches the
direction and rough magnitude the PR author reported on their own machine (`x86_64` AVX2: -8% to
-25%; `aarch64` NEON: -4% to -50%).

## Does it change behavior/output?

**This is a real fix to the tested project's own source** (`sparse_strips/vello_cpu/src/fine/...`)
- `patch.diff` is included and mandatory, and is present.

- **Bit-exact?** No - characterized below.
- **Snapshot-test pixel differences (expected, matches what the PR author already found and
  fixed):** running `cargo test --release -p vello_cpu -p vello_sparse_tests` before and after
  this patch (git-lfs was not preinstalled in this sandbox; a standalone `git-lfs` binary was
  downloaded and `git lfs pull`'d for this candidate specifically so the snapshot tests could run
  against real reference PNGs instead of LFS pointer text, unlike some earlier candidates in this
  workspace):
  - **Before** (pristine `base_commit`): 3541 passed, **86 failed**, 1230 ignored (4857 total in
    `vello_sparse_tests`); `vello_cpu`'s own 87 unit tests + 1 doctest all passed. All 86
    pre-existing failures are `_hybrid`-suffixed tests requiring a working GPU/wgpu backend, not
    available in this sandbox - confirmed unrelated to this patch (identical failing set before
    touching anything).
  - **After** (patched): 3522 passed, **105 failed**, 1230 ignored. **19 new failures**, all
    `_cpu_*`-suffixed snapshot-pixel-diff mismatches: `glyph::glyphs_bitmap_noto_cpu_f32_scalar_cached`,
    `glyph::glyphs_transform_composition_rows_bitmap_{,hinted_}cpu_f32_scalar_cached`,
    `gradient::linear::gradient_linear_with_{downward,upward}_y_cpu_{f32_scalar,multithreaded}`,
    `gradient::linear::gradient_linear_with_y_reflect_cpu_{f32_scalar,multithreaded}`,
    `gradient::linear::gradient_linear_with_y_repeat_cpu_{f32,u8}_avx2`,
    `gradient::radial::gradient_radial_spread_method_{reflect,repeat}_cpu_{f32_scalar,multithreaded}`.
    This is exactly the same set of tests whose reference snapshots the upstream PR itself updated
    (its own diff touches `glyphs_bitmap_apple*.png`, `glyphs_transform_composition_rows_bitmap*.png`,
    `gradient_linear_with_*.png`, `gradient_radial_spread_method_*.png`, plus a `#[vello_test(diff_pixels
    = 15)]` threshold bump on `gradient_linear_with_y_repeat` in `vello_sparse_tests/tests/gradient.rs`)
    - this candidate's `patch.diff` deliberately excludes those binary snapshot-PNG updates (see
    "What's not done" below), so these 19 tests fail here purely because the *old* reference PNGs
    no longer match the (slightly, sub-ULP-accumulation-different) new pixel output, not because
    the new output is wrong. The PR author's own description: "Nearly all tests just had a max
    channel delta of 1 and only very few changed pixels" (one test needed the larger `diff_pixels =
    15` threshold because of near-every-pixel color changes at sharp gradient inflection points).
- **A genuine new crash, not present upstream or in the current `patch.diff`'s own PR
  description/file list** (see "Incidental bug found" below): 4 of the 19 additional failures are
  not pixel-diff mismatches but panics: `image::image_bicubic_with_translation_cpu_{f32,u8}_avx2`
  and `image::image_bilinear_with_translation_cpu_{f32,u8}_avx2` panic with `index out of bounds:
  the len is 4 but the index is 4294967295` (`u32::MAX`, i.e. a wrapped `-1`) inside
  `Pixmap::sample_idx` (`sparse_strips/vello_common/src/pixmap.rs:378`). Confirmed deterministic
  and reproducible in isolation (`cargo test --release -p vello_sparse_tests --test tests --
  image::image_bilinear_with_translation_cpu_u8_avx2 --exact`). Both crashing tests use
  `Extend::Reflect` with a translating affine transform (`Affine::scale(5.0) *
  Affine::translate((10.0, 10.0))`) on a 2x2 source image - none of the upstream PR's own
  snapshot-PNG updates touch any `image_bilinear`/`image_bicubic` snapshot, meaning this specific
  crash was not caught by the PR author's own test run before this candidate's own run surfaced
  it (plausibly because their toolchain/platform didn't hit the exact same floating-point rounding
  at the reflect boundary, or their own test pass simply didn't include this specific pair). Root
  cause (not fixed as part of this candidate, see below): `fine/common/image.rs`'s `extend()`
  function's `Extend::Reflect` branch does a bit-level nudge (`m_bits - bias_in_ulps.to_int()`)
  that its own comment says relies on `m_bits` being exactly `0` whenever `bias_in_ulps` is
  non-zero ("since our `max` is always an integer number, `u` and `s` must also be an integer
  number and thus `m_bits` must be 0"); this candidate's patch changes how the fed-in position is
  accumulated (from per-chunk `splat_pos` off a scalar `f64` to per-chunk `f32` `i * step + base`
  SIMD arithmetic), which is enough of a floating-point rounding change to break that invariant
  for this specific translated/reflected case, producing a wrapping `u32` subtraction and
  ultimately an out-of-bounds pixmap index.
- **Is this a breaking change?** No public API changed (all touched types/functions are
  crate-internal or `pub(crate)`). The pixel-diff mismatches are not user-visible API breaks, but
  the crash above **is a real, user-triggerable panic** for anyone rendering a reflected,
  translated image with AVX2 - see "Incidental bug found".

## Why the result looks the way it does

Same mechanism the PR author already describes: the old code called `splat_pos` (broadcast +
compute a full linear ramp of per-lane offsets) once per 8/16-pixel chunk, discarding and
recomputing lane offsets that are actually a fixed, precomputable step from one chunk to the next.
The patch computes that ramp once and then advances by a single SIMD `add`/`mul_add` per chunk,
letting the compiler keep the running position live across loop iterations (no more re-deriving it
from a scalar every time) - a strictly cheaper per-chunk operation for every image/gradient
painter this candidate measured, consistent with the PR's own before/after numbers.

## Incidental bug found

Not folded into `___BUGHUNT___/` directly by this candidate (per this workspace's own convention:
an incidental non-performance finding from a perf-optimize session gets noted in the session's
final report for the batch-level transcription step to fold in, rather than written here
directly, to avoid a race with sibling candidates editing the same shared file concurrently). See
"Does it change behavior/output?" above for the full repro (`image_bilinear_with_translation`/
`image_bicubic_with_translation`, AVX2, `Extend::Reflect` + translating transform,
`index out of bounds: the len is 4 but the index is 4294967295` panic in `Pixmap::sample_idx`).
Checked `___BUGHUNT___/SUMMARY.md` first (candidates 006/009/021 are the closest existing
image-sampling findings in that file - zero-sized-pixmap panic, vello_hybrid pad-extend blend
order, and minification aliasing, respectively) - none of them match this mechanism, so this is a
genuinely new finding, specific to this **not-yet-merged** PR's own patch (it does not reproduce
against pristine `base_commit`, confirmed by the "before" test run above having zero failures in
this area).

## What's not done / open questions

- **This candidate's own `patch.diff` deliberately excludes the upstream PR's binary snapshot-PNG
  updates and does not fix the crash above.** The PR's own diff (fetched via `gh pr diff 1728`)
  updates 12 reference PNGs (all Git-LFS-tracked) plus one test-threshold bump; the PNG hunks
  could not be applied in this sandbox (GitHub's diff of an LFS-tracked file only shows the LFS
  *pointer* text, and this candidate's worktree had real, `git-lfs pull`'d binary PNG content
  checked out for correctness testing purposes - see above - so the pointer-text hunks no longer
  match and were excluded before capturing `patch.diff`). The one plain-text hunk (the
  `#[vello_test(diff_pixels = 15)]` bump in `vello_sparse_tests/tests/gradient.rs`) applied cleanly
  and is included.
- **The AVX2 `Extend::Reflect` + translation crash is a real, unresolved bug in the PR as fetched**
  - a maintainer taking this patch would need to either fix `extend()`'s reflect-branch bit-nudge
  invariant to tolerate the new position-accumulation path's rounding, or re-derive the position
  accumulation to preserve whatever exact-integer property the old scalar-`f64`-then-`splat_pos`
  path happened to guarantee. Not attempted here - out of scope for measuring the *performance*
  claim, and a correctness fix should be reviewed/tested independently rather than folded into a
  perf candidate's own patch.
- Only `sse4_2`/`avx2` were measured (this machine is `x86_64`); the PR's own NEON numbers were not
  independently reproduced here.
- `many_stops`/`transparent`/gradient `extend::{pad,repeat,reflect}` and image `extend::{pad,repeat,
  reflect}` sub-benchmarks (also present under `--features extended`) were not included in the
  measured set above, to keep the run to a reasonable size - no reason to expect them to behave
  differently from the measured `opaque`/`quality`/`transform` sub-benchmarks, since they go
  through the same `PaintPositions`/`PlainPaintPositions` machinery.
