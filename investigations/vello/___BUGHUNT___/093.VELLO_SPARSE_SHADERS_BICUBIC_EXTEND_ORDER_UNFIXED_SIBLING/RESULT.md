# vello_hybrid: bicubic (and external-texture bicubic) image sampling applies Extend once too early, disagreeing with vello_cpu

Candidate 009 confirmed and fixed this exact bug class (Extend applied once, upfront, to a single
already-averaged sample position instead of extending each neighboring tap independently) for the
bilinear (`ImageQuality::Medium`) path, and explicitly named `bicubic_sample`
(`helpers/image.wesl`) and `external_bicubic_sample` (`helpers/external_texture.wesl`) as an
acknowledged, still-open, unfixed sibling gap - confirmed both still took unused
`_extend_modes`/`_image_padding` parameters and unconditionally clamped every one of their 16 taps
to `[image_offset, image_offset + image_size - 1]` in the current tree (009's own fix was never
applied to the main tree, only recorded in its own candidate folder). This candidate independently
confirms and fixes the bicubic sibling.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `render.wesl`'s `PAINT_TYPE_IMAGE` branch + `helpers/image.wesl`/`helpers/external_texture.wesl` directly | current `HEAD` (75168314) | Confirmed the exact mechanism candidate 009 already named: `extended_xy` is computed once via a single `extend_mode()` call, then both `bicubic_sample` and `external_bicubic_sample` take that single already-extended point and clamp all 16 taps (`coords + {-1.5,-0.5,0.5,1.5}` per axis) to atlas/image bounds, instead of extending each tap independently the way `vello_cpu`'s `extend_x!`/`extend_y!` macros do (`vello_cpu/src/fine/common/image.rs`, `QUALITY == 2` branch) |
| Standalone differential repro (own `Cargo.toml`, public `vello_common`/`vello_cpu`/`vello_hybrid` API, real `wgpu::Device` via the sandbox's software Vulkan ICD), mirroring 009's own repro shape but for `ImageQuality::High` + `Extend::Repeat` with an 8px-wide, maximally-distinct-columns probe image sampled across 8 repeat periods | release build, `Affine::translate((0.5,0.5))`, `Extend::Repeat` in x / `Extend::Pad` in y | Reproduced cleanly and repeatably: 32/64 probe-row pixels differ by more than the project's own bicubic FP tolerance (2, see below) between vello_cpu and vello_hybrid, concentrated exactly at each `Extend::Repeat` period boundary - reproduced 3/3 runs, bit-for-bit identical each time |
| Applied a bounded fix (per-tap `extend_mode` in `bicubic_sample`/`external_bicubic_sample`, mirroring 009's `bilinear_sample` fix) and re-ran the repro | release build | Fixed the target bug (0/64 mismatches beyond tolerance 2), but surfaced a **new** regression in the project's own test suite: `image_bicubic_identity_hybrid` (`Extend::Reflect`, `ImageQuality::High`, a 2x2 image at `Affine::IDENTITY`) started failing with large (~200/255) per-channel diffs |
| Root-caused the regression with temporary debug instrumentation (a scoped `eprintln!` in `vello_cpu`'s own `FilteredImagePainter::next`, reverted before finishing) comparing `vello_cpu`'s actual per-tap extended values against the GPU's `extend_mode()` formula by hand for the same raw positions | debug build, `cargo test -p vello_sparse_tests --test tests --release -- image_bicubic_identity --nocapture` | Found the GPU's shared `extend_mode()` (`helpers/extend.wesl`) lacks two safety behaviors `vello_cpu`'s own `extend()` has: a `.min(max - 1.0)` clamp for Repeat/Reflect (their own comment: "In certain edge cases, we might still end up with a higher number"), and, for Reflect specifically, a 1-ULP-down bias in the "mirrored" half of the period that avoids landing exactly on a boundary and flooring to the texel one past the edge. The old single-upfront-extend code never hit these edge cases in practice (it also had a redundant safety `clamp()` to atlas bounds); this candidate's new per-tap extension calls `extend_mode()` with far more varied offsets, hitting them directly |
| Added a new `extend_mode_precise()` helper (in `helpers/extend.wesl`) with those two corrections, used only by the new per-tap `bicubic_sample`/`external_bicubic_sample` calls - **not** by modifying the shared `extend_mode()` itself | release build, full `cargo test -p vello_sparse_tests --test tests --release` | First attempt (modifying `extend_mode()` directly) broke 6 *different*, previously-passing tests (`image_bilinear_2x_scale_hybrid`, `_5x_scale_hybrid`, `_10x_scale_hybrid`, `_10x_scale_2_hybrid`, `_with_rotation_hybrid`, `_with_translation_hybrid` - all `Extend::Reflect`, still using the old, unfixed single-upfront-extend bilinear path, which happened to rely on `extend_mode()`'s un-biased behavior). Isolating the correction into a separate `extend_mode_precise()` used only by the new bicubic per-tap code fixed `image_bicubic_identity_hybrid` without touching any of those 6 |

No sanitizer/miri/fuzzing was used - this is a pure logic/arithmetic discrepancy between two
renderer backends' shader code (the same bug class as 009), not memory-unsafety or a crash, so a
debug/release-build differential comparison against `vello_cpu` (the project's own established
oracle for this class of bug) plus targeted instrumentation to root-cause the follow-on regression
was the correct and sufficient tool per `HEURISTICS.md`'s "logic bugs" guidance.

## Status

**Confirmed**. Reproduces deterministically 3/3 runs (release build, software Vulkan/swiftshader
adapter, no ASLR/thread-scheduling/hash-order dependency - both backends are pure functions of the
scene description) via the standalone `repro/` (own `Cargo.toml`, public
`vello_common`/`vello_cpu`/`vello_hybrid` API, a real `wgpu::Device`/`wgpu::Queue`).

**Fully reproducible**: **Yes**. `repro/` drives the real, public rendering entry points of both
backends end-to-end - `vello_cpu::RenderContext::{set_paint_transform,set_paint,fill_rect,flush,
render}` and `vello_hybrid::{Scene::{set_paint_transform,set_paint,fill_rect},Renderer::{upload_image,
render}}` against a real `wgpu::Device`/`wgpu::Queue` (Vulkan, via a real GPU or a software ICD).
This is a single-scene, differential comparison between two rendering backends within one process,
not an interaction between two separate participants exchanging data over a boundary (no peer/
caller to fake), so `CONFIRMATION.md` section 11's two-participant bar does not apply here.

## Class

logic

## Root cause

Two layered issues, both in `sparse_strips/vello_sparse_shaders/shaders/`:

1. **The bug this candidate is about**: in `render.wesl`'s `fs_main` (`PAINT_TYPE_IMAGE` branch),
   the local sample position is extended **once**, before dispatching to a per-quality sampling
   helper:
   ```wgsl
   let extended_xy = vec2<f32>(
       extend_mode(local_xy.x + offset, image_extend_modes.x, image_size.x),
       extend_mode(local_xy.y + offset, image_extend_modes.y, image_size.y)
   );
   ```
   For the High-quality (bicubic) branch, this single already-extended point was shifted and
   handed to `bicubic_sample`/`external_bicubic_sample`, which computed all 16 neighboring taps
   (`coords + {-1.5,-0.5,0.5,1.5}` per axis) by simply `clamp()`-ing each offset tap to the
   image's atlas bounds - never re-applying `Repeat`/`Reflect` wraparound to the individual taps.
   Deep past an `Extend::Repeat` image's right edge, the single upfront `extend_mode` call wraps
   the base point back into range, but the 16 taps around that one wrapped point then get clamped
   to the *nearest* edge instead of wrapping independently - producing a smeared, duplicated-edge
   blend at each repeat period boundary instead of a correct cross-boundary blend.

   `vello_cpu`'s equivalent code (`vello_cpu/src/fine/common/image.rs`,
   `FilteredImagePainter::next`, the `QUALITY == 2` bicubic branch) instead extends each of the 16
   neighboring sample positions independently (`extend_x!`/`extend_y!` macros with
   `OFFSETS = [-1.5, -0.5, 0.5, 1.5]`, applied to the *raw*, not-yet-extended position) - producing
   the correct cross-boundary blend.

2. **A follow-on issue this candidate's own fix surfaced** (not present in the original,
   unfixed code, because the old code's redundant atlas-bounds `clamp()` masked it): the shared
   `extend_mode()` helper (`helpers/extend.wesl`) computes Repeat/Reflect via
   `abs(t/max - 2*round(0.5*t/max)) * max`, which can land exactly on the boundary value `max`
   (an index one past the last valid texel) in edge cases, and additionally lacks `vello_cpu`'s
   own `extend()` function's deliberate 1-ULP-down bias in the "descending" (mirrored) half of a
   Reflect period. `vello_cpu`'s `extend()` has both corrections (its own comment: "In certain
   edge cases, we might still end up with a higher number"), sourced from
   <https://github.com/google/skia/blob/220738774f7a0ce4a6c7bd17519a336e5e5dea5b/src/opts/SkRasterPipeline_opts.h#L3274-L3290>.
   The old single-upfront-extend call almost never hit this edge case in practice (only one tap,
   plus a redundant safety clamp downstream); calling `extend_mode()` per-tap, 16 times with
   offsets up to ±1.5 texels, hits it far more often - most visibly for a 2x2 image at
   `Affine::IDENTITY` with `Extend::Reflect`, where the destination pixel grid lands exactly on
   reflection boundaries.

## Introduced by

805986f1 "[vello_hybrid] Image rendering (#1064)" (2025-07-02) - the same commit candidate 009
identified for the bilinear sibling, which first added `vello_hybrid`'s image-rendering
fragment-shader code (`bicubic_sample` included) with the single-upfront-`extend_mode` pattern.
Confirmed via `git log -S "Sample 4x4 grid around coords" -- sparse_strips/vello_sparse_shaders`,
which finds the pattern present as far back as #1064's own diff and continuously present in every
later commit that touched the file (the code has since moved during a WESL migration/refactor:
af1d0917 "refactor: migrate sparse shaders to WESL (#1767)", 522c4ed0 "refactor: split WESL shaders
into helper modules (#1818)" - the arithmetic itself is unchanged since #1064).

## The fix

Two changes, both in `sparse_strips/vello_sparse_shaders/shaders/`:

1. **`helpers/image.wesl`**: `bicubic_sample` now takes the raw, un-extended local sample position
   and independently extends each of its 16 neighboring texel positions (mirroring `vello_cpu`'s
   `extend_x!`/`extend_y!` macros), instead of clamping 16 taps around one already-extended point.
   The now-fully-unused `_image_padding` parameter was dropped (it was already dead before this
   change). `render.wesl`'s `IMAGE_QUALITY_HIGH` branch now passes the raw `local_xy + offset`
   instead of the already-extended `image_offset + extended_xy`.
2. **`helpers/external_texture.wesl`**: the identical fix for `external_bicubic_sample` (used for
   `ImageSource::ExternalTexture` rather than atlas-backed images), and `sample_external_image`'s
   signature extended to pass through both the already-extended coordinate (still used unchanged
   by the Medium/Low branches, out of scope for this candidate) and the new raw local coordinate +
   extend modes (used only by the High/bicubic branch).
3. **`helpers/extend.wesl`**: a new `extend_mode_precise()` helper, used only by the two per-tap
   call sites above, adding the `.min(max - 1.0)` safety clamp and the Reflect 1-ULP bias that
   `vello_cpu`'s own `extend()` has (see "Root cause" #2). Deliberately **not** merged into the
   existing, widely-used `extend_mode()` - doing that first broke 6 other, unrelated, previously-
   passing tests that exercise the still-unfixed single-upfront-extend bilinear path (see "What was
   tried" table above).

This is sufficient (not just papering over the one input that first triggered it) because it was
verified against multiple inputs in the same bug class, not just one: the original `Extend::Repeat`
+ `ImageQuality::High` repro (0/64 mismatches beyond tolerance after the fix, was 32/64 before), the
`image_bicubic_identity_hybrid` regression this fix's first draft introduced (`Extend::Reflect` +
`ImageQuality::High` + a 2x2 image at `Affine::IDENTITY` - now passes), and the 6 `image_bilinear_*`
tests that first draft broke (all now pass again, confirming `extend_mode_precise` doesn't leak
into the unrelated bilinear path).

- `repro/`: before the fix, `REPRODUCED (CPU and Hybrid disagree beyond the project's own bicubic
  FP tolerance under Extend::Repeat, ImageQuality::High)`, exit code 1, reproduced 3/3 runs
  (bit-for-bit identical: 24/64 mismatches on the probe row each time; the repro tolerates ±2 per
  channel, matching the project's own `cpu_u8_tolerance`/`hybrid_tolerance` conventions for
  bicubic-interpolation tests - e.g. `vello_sparse_tests/tests/glyph.rs`'s `cpu_u8_tolerance = 3`
  cases, comment: "likely due to having to use bicubic interpolation"). After applying
  `patch.diff`: `NOT REPRODUCED (CPU and Hybrid agree within tolerance 2)`, exit code 0, 3/3 runs.
  See `output.txt` for full captured before/after pixel dumps.
- Project test suite (`cargo test -p vello_sparse_tests --test tests --release`, git-lfs snapshot
  PNGs resolved for a genuine run rather than skipped):
  - **Before**: 3627 passed / 0 failed / 1230 ignored.
  - **After**: 3626 passed / **1 failed** / 1230 ignored. The one new failure,
    `external_texture::tests::external_texture_skewed_hybrid`, is a `hybrid_only` snapshot test
    (no live CPU cross-check - it only compares against a static reference PNG) using
    `Extend::Pad` + `ImageQuality::High` + a skewed transform on an external-texture sprite. The
    diff (max per-channel difference 54/255, 154/9216 pixels, confined to a thin anti-aliased
    sliver at the sprite's skewed edge - see `diffs/external_texture_skewed_hybrid.png` copied
    into `output.txt`'s narrative) is the expected consequence of `external_bicubic_sample` now
    computing its cubic-weight fraction from the true, un-clamped sub-pixel position (matching
    `vello_cpu`'s own `x_fract`/`y_fract`) instead of the old position derived from the
    already-Pad-clamped coordinate - every *other* test exercising the same bicubic code path with
    a live CPU comparison passes. This looks like a stale reference-image artifact of a genuine
    correctness improvement (see "What's not done" below), not a new regression, but this session
    did not regenerate/bless a new reference PNG for it.

## What's not done / open questions

- `external_texture_skewed_hybrid`'s reference PNG (`sparse_strips/vello_sparse_tests/snapshots/
  external_texture_skewed.png`) most likely needs regenerating against the fixed shader code as
  part of landing this patch for real - not done in this session, since blessing a new "golden"
  image is a maintainer decision, not something to do unilaterally from a bug-hunt session. A
  maintainer applying this patch should re-run that one test, visually confirm the new output
  still looks correct (the diff shown in `output.txt` suggests it does - same checkmark icon, tiny
  edge AA difference), and update the snapshot.
- `bilinear_sample` (`ImageQuality::Medium`) and `external_bilinear_sample` remain unfixed (the
  same single-upfront-extend bug, candidate 009's own explicitly-acknowledged scope) - out of
  scope for this candidate, which is specifically about the bicubic (`ImageQuality::High`) sibling.
  Applying 009's own patch for `bilinear_sample` would let it also benefit from this candidate's
  new `extend_mode_precise()` helper if a future session finds a similar boundary-precision gap
  there.
- The `LOW` (nearest-neighbor) quality path was not touched or independently re-verified - it only
  ever samples a single texel from the single upfront-extended point, so it doesn't have the
  multi-tap issue this candidate is about, but wasn't specifically stress-tested here either.
- No fuzzing/miri run was performed or would be applicable here - this is a pure floating-point
  logic discrepancy between two renderer backends' shader code, not memory-unsafety or a crash.
