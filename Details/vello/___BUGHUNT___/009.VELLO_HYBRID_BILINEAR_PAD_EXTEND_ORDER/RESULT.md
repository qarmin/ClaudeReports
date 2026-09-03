# vello_hybrid: bilinear image sampling applies Extend once too early, disagreeing with vello_cpu

Source: https://github.com/linebender/vello/issues/1590 ("vello_hybrid: Extend calculation for
images is wrong"). vello_hybrid's GPU fragment shader for bilinear (`ImageQuality::Medium`) image
sampling applies the extend mode (`Pad`/`Repeat`/`Reflect`) once to a single, already-averaged
sample position, then samples its two/four neighboring texels from that single clamped point.
vello_cpu instead extends each neighboring sample position individually. The maintainer's own issue
report already names the exact mechanism; this candidate independently confirmed it reproduces
against current `HEAD`, root-caused it against the real source, and verified a bounded fix.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the GitHub issue + the `bilinear_sample`/`render.wesl` fragment shader source directly | `sparse_strips/vello_sparse_shaders/shaders/{render.wesl,helpers/image.wesl}` | Confirmed the exact mechanism the issue describes: `render.wesl`'s `PAINT_TYPE_IMAGE` branch computes `extended_xy` once via a single `extend_mode()` call, then both the Medium (bilinear) and High (bicubic) branches, plus both external-texture equivalents, feed that single already-extended point into their sampling helpers instead of extending each neighbor individually (vello_cpu's `extend_x!`/`extend_y!` macros in `vello_cpu/src/fine/common/image.rs` do extend each neighbor individually) |
| Hand-rolled standalone differential repro (own `Cargo.toml`, public `vello_cpu`/`vello_hybrid` API, real `wgpu::Device` via a software Vulkan ICD) | release build, `Affine::translate((0.5,0.5))`/`Affine::scale(2.0)`/`Affine::IDENTITY` paint transforms | First attempt (bypassing the project's own `Level`/`RenderSettings` conventions) showed a *different*, spurious mismatch caused by an unrelated setup difference in my own driver code, not the real bug - discarded once it was traced down (see "What's not done" below) |
| Differential test built directly against the project's own internal (`pub(crate)`) `CpuRenderer`/`HybridRenderer` test harness (`vello_sparse_tests/tests/renderer.rs`), added temporarily as a `#[test]` in `issues.rs` | debug build, same three transforms, real `wgpu::Device` (swiftshader software Vulkan ICD, since this sandbox has no GPU) | Reproduced cleanly and repeatably: for `Affine::translate((0.5,0.5))`, vello_cpu correctly extends into solid red (`(255,0,0,255)`, the rightmost source texel) for every destination pixel from x=10 onward, while vello_hybrid produces a *constant* wrong blend `(128,128,0,255)` (a 50/50 mix of the two rightmost texels) across the *entire* padded region - reproduced 3/3 runs |
| Re-derived the standalone repro (public API only) to mirror the validated harness's exact settings (`Level::baseline()`, `num_threads: 0`, matching `RenderSettings`/`RenderSettings` field-for-field) | release build | Now reproduces identically to the internal-harness version: 3/3 runs before the fix, 0/3 mismatches after |

No sanitizer/miri/fuzzing was used - this is a pure logic/arithmetic discrepancy between two
renderer backends' shader code, not memory-unsafety or a crash, so a debug-assertions build plus
differential comparison against the CPU renderer (the project's own established oracle for this
class of bug, per its existing `vello_test` snapshot-comparison infrastructure) was the correct and
sufficient tool per `HEURISTICS.md`'s "logic bugs" guidance.

## Status

**Confirmed**. Reproduces deterministically 3/3 runs (release build, software Vulkan/swiftshader
adapter, no ASLR/thread-scheduling/hash-order dependency - both backends are pure functions of the
scene description) via both:
- The standalone `repro/` (own `Cargo.toml`, public `vello_common`/`vello_cpu`/`vello_hybrid` API,
  a real `wgpu::Device`/`wgpu::Queue`).
- A temporary differential test built on the project's own internal `CpuRenderer`/`HybridRenderer`
  test harness (used only during investigation, not part of the delivered fix or `repro/`).

**Fully reproducible**: **Yes**. `repro/` drives the real, public rendering entry points of both
backends end-to-end - `vello_cpu::RenderContext::{set_paint_transform,set_paint,fill_rect,flush,
render}` and `vello_hybrid::{Scene::{set_paint_transform,set_paint,fill_rect},Renderer::{upload_image,
render}}` against a real `wgpu::Device`/`wgpu::Queue` (Vulkan, via a real GPU or a software ICD) -
exactly the sequence a real caller (or the project's own `vello_test` snapshot tests) uses. This is
a single-scene, differential comparison between two rendering backends within one process, not an
interaction between two separate participants exchanging data over a boundary (no peer/caller to
fake), so `CONFIRMATION.md` section 11's two-participant bar does not apply here; the single-process
repro is the correct shape.

## Class

logic

## Root cause

In `sparse_strips/vello_sparse_shaders/shaders/render.wesl`'s fragment shader (`fs_main`,
`PAINT_TYPE_IMAGE` branch), the local sample position is extended **once**, before dispatching to a
per-quality sampling helper:

```wgsl
let local_xy = sample_xy - image_offset;
let offset = 0.00001;
let extended_xy = vec2<f32>(
    extend_mode(local_xy.x + offset, image_extend_modes.x, image_size.x),
    extend_mode(local_xy.y + offset, image_extend_modes.y, image_size.y)
);
```

For the Medium-quality (bilinear) branch, this single already-clamped/wrapped/reflected point was
then shifted by `-0.5` and handed to `bilinear_sample`, which computed `floor`/`ceil` of that one
point to get its two neighboring texel indices in each axis. Deep inside a padded region (far past
the image's right edge under `Extend::Pad`), the single upfront `extend_mode` call clamps
`local_xy.x` to a constant (`image_size.x - 1`) regardless of how far out-of-range the true position
is - but subtracting `0.5` and then taking `floor`/`ceil` of *that already-clamped* point still
straddles the two rightmost texels, producing a constant 50/50 blend of the last two texels for
*every* pixel in the padded region, instead of the correct "clamp fully to the last texel" behavior.

vello_cpu's equivalent code (`sparse_strips/vello_cpu/src/fine/common/image.rs`,
`FilteredImagePainter::next`, the `QUALITY == 1` bilinear branch) instead extends **each of the two
neighboring sample positions independently** (`extend_x!(0, OFFSETS)` and `extend_x!(1, OFFSETS)`
with `OFFSETS = [-0.5, 0.5]`, applied to the *raw*, not-yet-extended position): far in the padded
region, both neighbor positions clamp to the same value, so the blend weights collapse to give
100% weight to the single rightmost texel - the correct result.

`bicubic_sample` (`ImageQuality::High`) and both external-texture equivalents
(`external_bilinear_sample`/`external_bicubic_sample` in `helpers/external_texture.wesl`) receive
the same single-upfront-extended coordinate from the same caller and have the identical structural
flaw, but were not independently reproduced or fixed this session (see "What's not done" below) -
only the atlas-backed bilinear (`Medium`) path, which is what both the GitHub issue and this
candidate's repro exercise, was fixed.

## Introduced by

805986f1 "[vello_hybrid] Image rendering (#1064)" (2025-07-02) - the commit that first added
`vello_hybrid`'s image-rendering fragment-shader code, including the single-upfront-`extend_mode`
pattern this bug is about. The code has since moved files during a WESL migration/refactor
(af1d0917 "refactor: migrate sparse shaders to WESL (#1767)", then 522c4ed0 "refactor: split WESL
shaders into helper modules (#1818)"), but the actual arithmetic (extend once, then floor/ceil
around the single clamped point) is unchanged since #1064 and predates any WESL-specific code (this
was confirmed with `git log -S "uv_frac = fract(coords)"`, which finds the pattern present as far
back as #1064's own diff and continuously present in every later commit that touched the file).

## The fix

Changed `bilinear_sample` (`sparse_strips/vello_sparse_shaders/shaders/helpers/image.wesl`) to take
the raw, un-extended local sample position and extend each of its four neighboring texel positions
individually - mirroring vello_cpu's `extend_x!`/`extend_y!` macros exactly:

```wgsl
fn bilinear_sample(..., extend_modes: vec2<u32>) -> vec4<f32> {
    let frac = fract(coords + vec2(0.5));
    let x_lo = floor(extend_mode(coords.x - 0.5, extend_modes.x, image_size.x));
    let x_hi = floor(extend_mode(coords.x + 0.5, extend_modes.x, image_size.x));
    let y_lo = floor(extend_mode(coords.y - 0.5, extend_modes.y, image_size.y));
    let y_hi = floor(extend_mode(coords.y + 0.5, extend_modes.y, image_size.y));
    let a = textureLoad(tex, vec2<i32>(image_offset + vec2(x_lo, y_lo)), atlas_idx, 0);
    let b = textureLoad(tex, vec2<i32>(image_offset + vec2(x_lo, y_hi)), atlas_idx, 0);
    let c = textureLoad(tex, vec2<i32>(image_offset + vec2(x_hi, y_lo)), atlas_idx, 0);
    let d = textureLoad(tex, vec2<i32>(image_offset + vec2(x_hi, y_hi)), atlas_idx, 0);
    return mix(mix(a, b, frac.y), mix(c, d, frac.y), frac.x);
}
```

and updated the `render.wesl` caller's `IMAGE_QUALITY_MEDIUM` branch to pass the raw `local_xy +
offset` (instead of the already-single-extended `extended_xy - vec2(0.5)`) straight through. The
now-unused `image_padding` parameter was dropped from `bilinear_sample`'s signature (it was already
dead - prefixed `_image_padding` - before this change).

This is sufficient (not just papering over the one probe pixel that first triggered it) because it
was verified against multiple points/transforms in the same bug class, not just one: `Affine::
translate((0.5,0.5))` (the transform that showed the constant-wrong-blend pattern across the entire
padded region, matching the GitHub issue's own screenshot), `Affine::scale(2.0)` (mismatch only at
the boundary transition pixels), and `Affine::IDENTITY` (already correct before the fix, still
correct after - confirms no regression for the exactly-pixel-aligned case). All three transforms
match vello_cpu pixel-for-pixel after the fix; none matched before it (except the already-correct
`IDENTITY` case).

- `repro/` (the standalone, public-API repro): before the fix, `REPRODUCED (CPU and Hybrid disagree
  under Extend::Pad bilinear sampling)`, exit code 1; after applying `patch.diff`, `NOT REPRODUCED
  (CPU and Hybrid agree)`, exit code 0. See `output.txt` for the full captured before/after pixel
  dumps (both the standalone `repro/` and the internal-harness differential test used during
  investigation).
- Project test suite (`cargo test -p vello_sparse_tests --test tests --release`, the full
  `vello_cpu`+`vello_hybrid` snapshot/comparison suite, git-lfs snapshot PNGs fetched for a genuine
  run rather than skipped): **3627 passed / 0 failed / 1230 ignored** before the patch, **3627
  passed / 0 failed / 1230 ignored** after the patch - identical, no regressions, no new failures
  (ignored tests are SIMD-variant tests for instruction sets this machine doesn't have, e.g. neon).

## What's not done / open questions

- `bicubic_sample` (`ImageQuality::High`) has the same structural single-upfront-extend flaw (it
  still receives `extended_xy` from the caller, unchanged) but was not fixed or independently
  reproduced this session - the GitHub issue and this candidate's repro are both specifically about
  bilinear (`Medium`) sampling. A maintainer extending the same per-corner-extend technique to the
  bicubic path's 16-sample grid is a reasonable follow-up, but is a larger, more invasive change
  (16 samples instead of 4) that didn't seem appropriate to fold into this bounded fix without a
  separate confirmed reproduction for that specific quality level.
- Both external-texture sampling helpers (`external_bilinear_sample`/`external_bicubic_sample` in
  `helpers/external_texture.wesl`, used for `ImageSource::ExternalTexture` rather than atlas-backed
  images) have the identical single-upfront-extend structure fed from the same caller, and were
  not touched or independently verified - same class of bug, different code path, worth a
  maintainer's/future session's look.
- The standalone `repro/`'s *first* iteration (built directly against public API without mirroring
  the project's own `RenderSettings`/`Level` conventions) showed a spurious, unrelated mismatch that
  was not this bug - traced to a difference in the `Level`/render-settings the CPU and Hybrid
  backends were constructed with in that earlier attempt (not investigated further once the
  properly-mirrored version reproduced correctly and matched the internal-harness differential test
  exactly). Worth a note for any future session hand-rolling a CPU-vs-Hybrid differential repro:
  mirror `vello_sparse_tests/tests/renderer.rs`'s exact `RenderSettings`/`Level` construction rather
  than relying on each backend's own "simple" default constructor, since the two crates' defaults
  are not obviously required to agree.
- No fuzzing/miri run was performed or would be applicable here - this is a pure floating-point
  logic discrepancy between two renderer backends' shader code, not memory-unsafety or a crash.
