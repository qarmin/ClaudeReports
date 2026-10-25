# Image minification (scale < 1.0) aliases instead of being properly filtered (linebender/vello#1289)

`Image::encode_into` (`vello_common/src/encode.rs`) computes `x_advance`/`y_advance` from the
*inverse* of the paint transform - i.e. the per-output-pixel step in *source-image* space. When the
transform scales down (minification, e.g. `scale(0.125)` for an 8x downscale), these advances
become correspondingly large (8 source texels per output pixel for an 8x downscale). But
`vello_cpu`'s image painters (`fine/common/image.rs`) always sample a *fixed*-size neighborhood
around the current position - 2x2 for bilinear (`ImageQuality::Medium`), 4x4 for bicubic
(`ImageQuality::High`) - regardless of how large the advance step is. For any minification factor
above ~2x, this samples only a small, phase-dependent fraction of the source texels a correct
minification filter (a box filter / mipmap) should average, causing aliasing. This matches an
already-open upstream issue (#1289) verbatim, including its own worked 8x-downscale example; this
candidate independently re-derived and reproduced the same root cause via code reading before
finding the issue confirms it, then verified it with a real, standalone, deterministic repro and
attempted (and verified) a bounded fix for the common case.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading (`HEURISTICS.md`-style: trace how a per-pixel step/advance value is derived and consumed) | `vello_common/src/encode.rs` (`x_y_advances`, `Image::encode_into`) and `vello_cpu/src/fine/common/image.rs` (`FilteredImagePainter`, `NNImagePainter`) | Confirmed by reading: `x_advance`/`y_advance` scale with the inverse transform (grow for minification), but `FilteredImagePainter`'s bilinear/bicubic offsets (`[-0.5, 0.5]` / `[-1.5,-0.5,0.5,1.5]`) are compile-time constants, never scaled by the advance magnitude |
| Existing test-suite check | `sparse_strips/vello_sparse_tests/tests/image.rs` | Confirmed there is no existing test coverage for minification at all - every `image_bilinear_*x_scale`/`image_bicubic_*x_scale` test uses `Affine::scale(2.0)`, `5.0`, or `10.0` (magnification only); nothing exercises `scale < 1.0` |
| Manual boundary-value construction + real public API (`vello_cpu::RenderContext`) | A "striped" 512x512 source pixmap (a single bright column every 8 source pixels, otherwise black) downscaled via a real `Image` paint + `paint_transform` + `fill_rect` + `render`, debug build, both `ImageQuality::Medium` and `::High`, both an exact-integer (8x) and a non-integer (~8.4x) downscale factor | Reproduced immediately and dramatically: an ideal box-filtered result should be a spatially uniform ~31.9 (255/8); actual output was a uniform 128 (bilinear, exact 8x, +302%) / 136 (bicubic, exact 8x, +327%) / a highly non-uniform row ranging 0-245 with variance 4398 (bilinear, ~8.4x, phase-drifting) |
| Repeat-run check (`CONFIRMATION.md` section 1) | Same binary, 3 runs per case | 3/3 runs identical in every case (deterministic, no dependency on ASLR/thread scheduling) |

A debug-assertions build and manual boundary-value construction were sufficient to reproduce this
deterministically and dramatically on the first attempt; no sanitizer/miri/fuzzing was needed
(this is a pure logic/rendering-quality issue, not memory-unsafety), so the soft 3-approach cap in
`SKILL.md` was not exhausted.

## Status

**Confirmed**. Reproduces 3/3 runs, debug build, fully deterministic (identical numeric output
every run - no dependency on ASLR/thread scheduling/hash order). Also independently confirmed by
this project's own already-open issue #1289 (linked in `___BUGHUNT___/__issues_raw/issues/1289.json`),
whose own worked 8x-downscale example matches this candidate's finding exactly.

**Fully reproducible**: **Yes**. The repro drives the real, public `vello_cpu` API end-to-end
(`RenderContext::set_paint` with a real `Image`/`ImageSource::Pixmap` + `set_paint_transform` +
`fill_rect` + `flush` + `render`) - the same sequence any real `vello_cpu` caller uses. There is no
second participant to fake here: a source `Pixmap` with a given transform is itself the single
external input this finding is about (directly analogous to "a malformed file handed to a parser's
real public `open_file`" in `CONFIRMATION.md` section 11), so a single-process repro is the correct
shape.

## Class

logic

## Root cause

See the summary above. Concretely, in `sparse_strips/vello_common/src/encode.rs`,
`Image::encode_into` computes `x_advance`/`y_advance` from `transform.inverse()` via the shared
`x_y_advances` helper - correctly reflecting that minification means "each output pixel step covers
more than one source texel." But nothing downstream adapts the *sampling kernel size* to that step:
in `sparse_strips/vello_cpu/src/fine/common/image.rs`, `FilteredImagePainter::next` always samples
exactly 2 offsets per axis for bilinear (`OFFSETS: [f32; 2] = [-0.5, 0.5]`) or 4 for bicubic
(`OFFSETS: [f32; 4] = [-1.5, -0.5, 0.5, 1.5]`), regardless of how large `x_advance`/`y_advance` are.
For an 8x downscale, a correct minification filter should average all ~64 source texels each output
pixel's footprint covers; the actual code averages only the 4 (bilinear) or 16 (bicubic) texels
immediately around the *center* of that footprint, ignoring the rest entirely. Depending on where
those fixed offsets land relative to the source content's own spatial frequency (a function of the
phase of the advance step, not of the source content), this produces either a large systematic bias
(if the phase stays locked across the image, as happens for an exact-integer downscale factor - see
the "+302%"/"+327%" results above) or visible banding/moire (if the phase drifts, as happens for a
non-integer downscale factor - see the "variance = 4398" result above). `PlainNNImagePainter` and
`NNImagePainter` (nearest-neighbor, `ImageQuality::Low`) have the same structural gap, but nearest-
neighbor sampling is expected to alias by design at any scale, so this candidate focuses on the
filtered (`Medium`/`High`) painters, which are the ones a caller would reach for specifically to
avoid this kind of artifact.

## Introduced by

b7f82779 "Add support for rendering images (#919)" (2025-04-22) - the commit that first added
image rendering, including the very first version of `x_y_advances`/per-pixel step-based sampling
and a fixed small interpolation neighborhood. This is a best-effort attribution, not a precise
regression point: this is a missing feature (no minification/mipmap filtering was ever
implemented), not a regression introduced by later refactors. 33f4be83 "Align image rasterization
more with Skia implementation (#1029)" (2025-06-02) rewrote the exact bilinear/bicubic kernel math
to its current Skia-derived form (the `[-0.5, 0.5]`/`[-1.5,-0.5,0.5,1.5]` fixed offsets seen today)
but preserved the same fixed-kernel-size design; every commit in between and since has kept the
same structural gap. `git log -S`/`git blame` on `FilteredImagePainter`/`x_y_advances` confirms
these are the two commits that materially touched this logic; no single commit "introduced a
regression" in the sense of previously-correct behavior becoming wrong.

## The fix

Added a box-downsample pre-filtering step in `Image::encode_into`
(`sparse_strips/vello_common/src/encode.rs`), scoped to the case this session could verify safely:
an axis-aligned (unskewed/unrotated) minification of a directly-embedded `ImageSource::Pixmap`
(i.e. not yet resolved via `ImageResolver`/an atlas, and not an `ExternalTexture`). When
`x_y_advances` reports advances with no skew (`x_advance.y`/`y_advance.x` both ~0) and either axis's
advance magnitude is >= 2 (indicating at least a 2x minification on that axis), the source pixmap is
box-downsampled (block-averaging premultiplied RGBA8 texels) by the per-axis integer factor before
being used as the image source, and the transform/advances are recomputed against the smaller,
pre-averaged image. The existing fixed-size bilinear/bicubic kernel then operates on an
already-properly-filtered image instead of skipping most of the original footprint.

This was chosen over implementing a full mipmap chain (the issue's own suggested "progressive
downsampling") because a real mipmap needs: (1) a decision on where generated mip levels get
cached (once per unique `(image, LOD)` pair, ideally shared across draws/frames, not recomputed
per-draw) and (2) equivalent handling in `vello_hybrid`'s GPU/atlas-backed path
(`ImageSource::OpaqueId`/`ExternalTexture`), where the actual pixel data isn't available at
`encode_into` time at all (it's resolved later via `ImageResolver`, or lives in a GPU-side atlas
texture) - both of these are real architectural/caching decisions for the project maintainers, not
something to force through a single bug-hunt session's bounded fix. The box-downsample chosen here
is a real, verified improvement for the common single-process `ImageSource::Pixmap` case (exactly
what a `vello_cpu`-only caller with an in-memory image would hit), computed once per draw call at
encode time (not on the per-pixel hot path), at an extra cost proportional to the source image's
own pixel count (the same order of work as the resampling that already unavoidably happens once per
draw regardless).

Verified against all three cases in the repro (bilinear exact-8x, bilinear non-integer-~8.4x,
bicubic exact-8x) - per `CONFIRMATION.md` section 7, this varies both the quality mode and whether
the downscale factor is an exact integer (phase-locked bias) or not (phase-drifting banding), not
just the one input that first showed the bug. All three aliased before the fix (mean deviations of
+302%/-0% mean but variance 4398/+327% respectively vs the ideal uniform ~31.9) and all three
matched the ideal uniform value (32.00, 0% deviation, 0 variance) after it.

- Project test suite (`cargo test -p vello_cpu -p vello_common --lib`, this sparse_strips
  workspace): 159 passed / 0 failed / 1 ignored (vello_common) and 81 passed / 0 failed / 0
  ignored (vello_cpu), identical before and after the patch - no regressions, no new failures.
  `vello_sparse_tests`' own snapshot-comparison suite (which does cover `image_bilinear_*`/
  `image_bicubic_*` magnification cases) could not be run in this sandbox - the reference PNGs are
  Git-LFS-tracked and `git-lfs` is not installed here (see `___BUGHUNT___/README.md`'s documented
  limitation); since every existing snapshot test in `image.rs` uses `scale >= 2.0` (magnification),
  and the fix's `minification_factors` helper returns `None` (no-op, original code path preserved)
  whenever neither axis's advance exceeds 1 texel, magnification cases are structurally unaffected
  by this patch regardless.

## What's not done / open questions

- The fix only covers the directly-embedded `ImageSource::Pixmap` case with no skew/rotation.
  `ImageSource::OpaqueId` (atlas-cached images, resolved later via `ImageResolver`) and
  `ImageSource::ExternalTexture` are unaffected - the same aliasing this candidate reproduces still
  applies to those paths (in `vello_cpu` when an `ImageResolver` is used, and in `vello_hybrid`'s
  GPU path generally, which was not checked directly this session but shares the same "fixed 2x2/
  4x4 kernel, advance-independent" design in its own fragment shader per candidate
  009's own prior finding about `vello_hybrid`'s bilinear filtering).
- Skewed/rotated minifying transforms are also unaffected (deliberately excluded from
  `minification_factors` - a simple per-axis box filter is not correct once the source axes are no
  longer aligned with the output axes); those cases keep the pre-existing aliasing behavior.
- The box-downsample is recomputed on every `encode_into` call (i.e. potentially once per draw per
  frame for an animated/repeatedly-drawn image) rather than being cached across draws - a real
  mipmap-chain implementation would want to cache generated levels; this was out of scope for a
  bounded single-session fix.
- Edge blocks near the image boundary are averaged only over the source pixels actually present,
  not wrapped/reflected per the image's own `Extend` mode - a reasonable approximation, but not
  bit-exact with what a true `Extend`-aware box filter would produce at the very edge.
- No fuzzing/miri run was performed - the bug was directly, deterministically reproducible from a
  single boundary-value construction referencing an already-open upstream issue, so escalating to
  those tools was not needed per the soft cap in `SKILL.md`.
