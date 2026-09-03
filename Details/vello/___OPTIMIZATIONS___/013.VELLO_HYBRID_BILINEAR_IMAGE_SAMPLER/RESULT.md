<!-- rustc 1.98.0 (88d9e12ae 2026-08-18), cargo 1.98.0 (797e8a9bc 2026-08-05) -->

# VELLO_HYBRID_BILINEAR_IMAGE_SAMPLER

`vello_hybrid`'s WGSL fragment shader (`sparse_strips/vello_sparse_shaders/shaders/render.wesl`
via `helpers/image.wesl`) implements `ImageQuality::Medium` (bilinear) image sampling manually:
4 `textureLoad`s at the surrounding texel corners plus two `mix()`s, instead of the GPU's
built-in hardware bilinear sampler (`textureSample`/a `sampler` binding). The code even carries
a `// TODO: If we want to use native bilinear sampling for uploaded images, we can pass 1
instead of 0 here` comment next to `IMAGE_PADDING` and a matching `// TODO: add a fast path for
images where we are using bilinear sampling ... using GPU-native bilinear sampling` right at the
sampling call site. This candidate implements that TODO as a new opt-in `Scene::draw_image`
fast path, following the design of upstream's own open PR #1493 ("vello_hybrid: Faster bilinear
image rendering"), and measures it - informed by the same author's follow-up investigation PR
#1547, which found the raw native-sampler swap alone was a much smaller win than two *other*
changes (moving per-pixel atlas-dimension math to the vertex shader, and removing quality
branching) tried on top of it.

## What was tried

Implemented (see `patch.diff`): `EncodedImage::custom` bit flag + `Scene::draw_image(image,
rect, bilinear)` (bypasses the paint-transform API, forces `IMAGE_QUALITY_GPU_FAST_PATH = 3`),
`IMAGE_PADDING` bumped from 0 to 1 pixel of transparent atlas padding, a new `atlas_sampler`
(`wgpu::Sampler`, linear filtering) bind group entry, and a new fast-path branch in
`render.wesl`'s fragment shader using `textureSample(atlas_texture_array, atlas_sampler,
sample_xy, atlas_index)` with the UV normalized to `[0,1]` in the vertex shader - versus the
existing manual `bilinear_sample()` (4x `textureLoad` + `mix`) reached via
`set_paint(Image{sampler:{quality: Medium, ...}})` + `fill_rect`. Scope: wgpu backend only (not
webgl); the vendored `no_structs_in_fragment`/`no_dynamic_indexing` WESL lints were unaffected.

**Environment caveat (read before the numbers):** this sandbox has no GPU (`/dev/dri` absent,
`glxinfo`/`vulkaninfo` absent). The only available `wgpu` adapters are software rasterizers:
SwiftShader (Vulkan, via a `vk_swiftshader_icd.json` already set up at
`../tmp/swiftshader_icd/` from a prior session) and Mesa llvmpipe (GL). The GL/llvmpipe backend
was ruled out for timing: `device.poll(PollType::wait_indefinitely())` returned near-instantly
(~0.8ms/frame) regardless of shader content, which does not look like genuine GPU-work
synchronization on that backend in this wgpu version - so all numbers below are from the Vulkan
SwiftShader adapter, which did show consistent, repeatable, shader-content-dependent timing.
**A software rasterizer has no dedicated texture-sampling hardware**, so it cannot demonstrate
the mechanism upstream's PR #1493 measured its win from (dedicated GPU texture units). These
numbers characterize this specific software-Vulkan environment, not real GPU hardware.

A first bench pass accidentally measured the *wrong* thing: the grid used a 1:1-scale,
integer-translation-only transform per image, which `EncodedImage::encode_into` silently
downgrades from `Medium` to `Low` (nearest-neighbor) - and at exact 1:1 integer alignment,
bilinear and nearest-neighbor sampling are numerically identical anyway, so both modes produced
bit-identical output and the "manual" side was secretly doing the cheapest possible path. Fixed
by using a `1.0625x` scale + fractional (`+2.37`/`+2.61`) offset per image so both paths do
genuine bilinear interpolation (confirmed via the pixel-diff below, which is non-zero once this
was fixed). This is exactly the kind of confound MEASUREMENT.md warns about; see
`gpu_image_bench_diff.png` for visual confirmation the two paths differ once the confound was
removed.

Bench: 256 tinted 48x48 images (`GRID=16`, `IMG_SIZE=48`) drawn into a 1024x1024 target, 60
frames/run (first 10% discarded as warmup), `device.poll(wait_indefinitely)` after every frame
so the timer includes actual (emulated) GPU completion, not just submission. `bench/src/main.rs`
(candidate-local, not part of `patch.diff` - see "Does it change behavior/output?").

All numbers below are from the corrected bench (post transform-confound-fix, see above); an
earlier, degenerate pre-fix run is not reported here since it measured nearest-neighbor for both
variants, not bilinear (see the confound paragraph above for why it was discarded rather than
"corrected in place").

| Variant | single-core (`taskset -c 0`) min / p50 / mean, run 1 | same, run 2 (repeat) | multi-core (unpinned) min / p50 / mean, run 1 | same, run 2 (repeat) |
|---|---|---|---|---|
| baseline (manual `bilinear_sample`, `set_paint(Image)`) | 180.7 / 181.9 / 182.2ms | 184.1 / 185.5 / 185.8ms | 28.7 / 30.2 / 30.2ms | 29.0 / 31.7 / 31.6ms |
| patched (`Scene::draw_image`, `textureSample`) | 179.3 / 181.1 / 181.3ms | 179.1 / 180.9 / 181.1ms | 27.2 / 28.6 / 28.7ms | 28.2 / 29.6 / 29.6ms |

Single-core: essentially statistical parity, fast fractionally ahead in both runs (min/p50/mean
all slightly lower, but by less than the run-to-run variation between the two baseline repeats
themselves - e.g. baseline's own two runs differ by ~3.4ms at min). Multi-core: a small,
consistent, repeatable edge for the fast path (~2-5% lower min/p50/mean, same direction in both
runs, never reversed). A third, independent confirmation multi-core run - via the candidate's own
`just bench` recipe, fresh clone, no `taskset` - also put fast slightly ahead: 28.2ms vs 28.4ms
min, 29.56ms vs 29.70ms p50. `uptime` load average during these runs ranged roughly 6-25 on this
24-core machine (shared with sibling candidates' own `flock`-wrapped and non-`flock`-wrapped
work), consistent with the run-to-run spread observed above; `min` is the statistic trusted most
since it's least affected by transient contention.

## Result

In this software-Vulkan (SwiftShader) environment: **no significant difference on single-core**
(both ~179-186ms/frame, `fast` fractionally ahead but within the two baseline repeats' own
run-to-run spread) and a **small, repeatable multi-core edge for the native-sampler fast path**
(~2-5% lower min/p50/mean across 3 independent multi-core runs, never reversed).
Time only was measured (no RSS/allocation-count instrumentation attempted). This does not
confirm or refute upstream's own real-GPU numbers (PR #1493: manual bilinear 144.6ms/f -> native
104.2ms/f, ~28% faster, on a Samsung A05s mobile GPU) - a plausible reconciliation is PR #1547's
own finding that the *raw* sampler swap alone is a comparatively modest win on real hardware too,
with the bulk of that investigation's later, much larger wins (30-75%) coming from two changes
this candidate deliberately did not implement (hoisting per-pixel atlas-dimension math to the
vertex shader; removing image-quality branching from the fragment shader) - those are exactly
the sort of software-rasterizer-agnostic, instruction-count wins that plausibly *would* show up
in this environment too and are worth a follow-up candidate.

## Does it change behavior/output?

- **Bit-exact?** No, once the transform confound above is corrected. See
  `gpu_image_bench_diff.png` (max channel diff amplified 4x for visibility) diffed against
  `gpu_image_bench_manual.png`/`gpu_image_bench_fast.png`: **65,054 of 1,048,576 pixels (6.2%)
  differ**, concentrated in ~2px-wide rings at each image's edge, up to 67/255 per channel.
- **What changes, for whom, under what conditions:** `Scene::draw_image`'s fast path always
  samples transparent pixels just outside an image's bounds (via the new 1px transparent atlas
  padding + a hardware-sampled edge fade), whereas `set_paint(Image{sampler:{x_extend:
  Extend::Pad, ...}})` (the "manual" path, and the only way to get bilinear sampling before this
  patch) clamps to the edge pixel's color instead. This is an intentional, expected consequence
  of the fast-path's design (documented in `Scene::draw_image`'s own doc comment: "always sample
  transparent pixels outside," mirroring upstream's PR #1493 comment) - not a bug - but it means
  `draw_image` is not a drop-in replacement for `set_paint(Image) + fill_rect` when
  `Extend::Pad`/`Extend::Clamp`-style edge behavior is required; it only supports the "fade to
  transparent at the edge" behavior. `Extend::Repeat`/`Extend::Reflect` proper tiling is also not
  available through `draw_image` (the field is repurposed to select bilinear-vs-nearest
  instead).
- **Is this a breaking change?** No existing public API changes signature or behavior;
  `Scene::draw_image` is new, `IMAGE_PADDING` going from 0 to 1 only affects atlas memory layout
  (each image now reserves 1 extra transparent pixel border), not any existing rendering path's
  output (verified: `bilinear_sample`/`bicubic_sample`'s `image_padding` parameter is unused,
  `_image_padding: f32`, in the current shader - only the new fast path reads padding pixels).
- **Project test suite: failing before -> failing after.** `cargo test -p vello_common -p
  vello_cpu -p vello_hybrid --release --lib --bins --tests`: **363 passed / 0 failed / 1 ignored,
  identical before and after** (160+81+122 across the three crates, both runs). `cargo test -p
  vello_sparse_tests --release` (the golden-image/snapshot suite, which also exercises the real
  `vello_hybrid` wgpu backend): **89 passed / 3538 failed / 1230 ignored, identical set of
  failing test names before and after** (diffed the two full lists of `FAILED` test names -
  zero difference). All 3538 failures are pre-existing and environment-caused: this sandbox has
  no `git-lfs` binary, so every LFS-tracked reference-snapshot PNG under
  `vello_sparse_tests/tests/snapshots/` is still an LFS pointer-text file, and
  `image::load_from_memory` on it fails with `Unsupported(Format Unknown)` at
  `tests/util.rs:403` for every single snapshot-comparison test, in both the baseline and
  patched tree alike - confirmed identical, not a new regression.

## Why the result looks the way it does

The mechanism upstream measured its win from - a real GPU's dedicated texture-sampling hardware
doing the 4-tap-plus-blend work for free in parallel with everything else - simply doesn't exist
in a software (CPU-JIT-compiled) Vulkan implementation like SwiftShader: there, `textureSample`
lowers to roughly the same interpolation arithmetic the manual `bilinear_sample()` already does
by hand, so the two paths are close to equivalent in cost, and any remaining difference is more
likely explained by minor differences in generated-code shape and reduced fragment-shader
instruction count (dead branches skipped in the fast path, since the manual path's
`extend_mode`/`local_xy`/`offset` computation - moved into the `else` branch of the new
top-level `if` - is now skipped entirely for fast-path draws) than by anything resembling a
hardware sampler benefit. The small, consistent multi-core edge is plausibly this reduced
instruction count; the tail noise at times of high sibling-candidate system load swamps that
small a signal on single-core.

## What's not done / open questions

- Not measured on a real GPU (none available in this sandbox) - the headline upstream number
  (~28% faster on a real mobile GPU) is neither confirmed nor contradicted here, only shown to
  be environment-dependent.
- webgl backend not touched (scope limited to wgpu, since that's what this environment can
  exercise at all).
- PR #1547's two much larger follow-on changes (vertex-shader atlas-dimension hoisting; removing
  image-quality branching) were deliberately not implemented here (candidate scope is the raw
  sampler swap only, per the task) - flagged as a separate, likely more promising follow-up idea
  since their mechanism (fewer/cheaper instructions) should be visible even in software
  rasterization, unlike the pure-sampler-hardware win.
- No RSS/allocation measurement attempted (this change doesn't plausibly move either - it swaps
  shader instructions, not host-side allocations).
- `git apply --check` was verified against `base_commit` (7516831426fb70f36d62c521c5477f6a7265b36a)
  and the full `just test` cycle (clone -> build-baseline -> build-patched -> bench) was run
  end-to-end from a fresh clone in a scratch directory before finalizing this folder.
