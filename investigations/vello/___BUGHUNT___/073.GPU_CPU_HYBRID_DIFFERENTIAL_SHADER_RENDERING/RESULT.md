# GPU_CPU_HYBRID_DIFFERENTIAL_SHADER_RENDERING

Set out to actually run a real CPU-vs-GPU-vs-`vello_hybrid` differential rendering harness in this
sandbox (per `projects/vello.md`'s risk map: prior confirmed GPU shader bugs 011/014/017/020 were
all found by manual code reading, never by executing a real `wgpu` device), to see whether running
real shaders on a real device surfaces bugs manual reading can't catch. This is the same open
question candidate 065 left explicit ("a broader differential harness that also compares
`vello_hybrid`'s own sparse-strip GPU renderer against `vello_cpu`'s CPU renderer... was not
attempted this session and is a promising next step").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Re-derived candidate 065's already-recorded fix to `vello_tests/src/compare.rs` locally (session-only, not part of this candidate's own patch - see "What's not done" below), then ran the project's own `compare_gpu_cpu` harness plus a custom standalone differential harness (`vello_cpu::RenderContext` vs `vello_hybrid::Renderer`, driven by a real `wgpu::Device`) across all 22 scenes in `vello_example_scenes::get_example_scenes` | Release build, real Vulkan device (SwiftShader software ICD, per README.md) | 22/22 scenes render visually correctly and within small (<0.1 mean byte diff per channel, matching the project's own `DEFAULT_HYBRID_TOLERANCE`-scale tolerance) numeric divergence from `vello_cpu` - ordinary floating-point/algorithm-level noise between two independent implementations, not a defect. No bug found via the Vulkan backend across this scene set. |
| Same 22-scene differential sweep, forcing `WGPU_BACKEND=gl` (Mesa's `llvmpipe` software rasterizer) instead of Vulkan | Release build, real GL/GLES device | 20/22 scenes: same small-divergence range as Vulkan. The 2 image-bearing scenes (`image`, `multi_image`) instead came out almost **entirely blank** (mean byte diff 32/255 and 101/255 respectively, vs. 0.07/255 and 0.32/255 on Vulkan) - the uploaded photo content is missing, replaced by the render target's cleared background. Confirmed visually via dumped PNGs (a solid green/pink flower photo vs. a blank white image). |
| Minimal, isolated repro: upload a single small solid-red opaque `Pixmap` through `vello_hybrid::Renderer::upload_image` (real public API), `fill_rect` covering the whole canvas with it, render through a real `wgpu::Device` explicitly requested on `wgpu::Backends::GL` | Release build, `LIBGL_ALWAYS_SOFTWARE=1` (llvmpipe) | Reproduces cleanly: 100% of the 64x64 render target's pixels differ from the uploaded color by more than tolerance (i.e. the image is entirely absent), while the identical scene through `vello_cpu` renders the solid red rectangle correctly (0% mismatch). Repeated 3/3 times, deterministic every run. |
| Same minimal repro, same unpatched checkout, forcing `wgpu::Backends::VULKAN` instead of `Backends::GL` | Release build, SwiftShader software Vulkan ICD | 0% mismatch - the identical scene, same unpatched code, renders correctly on Vulkan. Confirms the bug is specific to wgpu's GL/GLES backend, not a general `vello_hybrid` defect. |

No sanitizer/miri applies here (this is a GPU-backend-selection/texture-classification logic bug,
not memory unsafety); the escalation used real differential rendering across increasingly narrow
scenes (broad gallery sweep -> two suspect scenes -> a single minimal isolated repro) instead.

## Status

**Confirmed**. Reproduces deterministically: 3/3 runs of the minimal standalone repro (release
build, real `wgpu::Backends::GL` device via Mesa's `llvmpipe`) all show the uploaded image
completely absent (100% pixel mismatch against the known solid-red input) before the fix, and 3/3
runs show it rendering correctly (0% mismatch) after. Independently corroborated by the broader
22-scene differential sweep (both image-bearing scenes go from "visually blank" to "matches
`vello_cpu`" after the same fix) and by the cross-check that the identical unpatched code renders
correctly on `Backends::VULKAN`.

**Fully reproducible**: **Yes**. The repro drives the real, unmodified, public
`vello_hybrid::Renderer::upload_image`/`Scene::fill_rect`/`Renderer::render` entry points through a
real `wgpu::Device`/`wgpu::Queue` (real GPU compute/render pipeline, real GLES driver via Mesa's
software rasterizer) end to end - the same public API and code path a real embedder targeting
native GL or WebGL would use. This is a single-process scenario (an image handed to the renderer's
own real public upload/render API); there is no second peer/caller whose behavior needed to be
faked, so CONFIRMATION.md section 11's bar is satisfied the same way as for a file handed to a
parser's own public entry point.

## Class

logic (a rendering-backend-selection defect: uploaded image content is silently lost/never sampled
on wgpu's GL/GLES backend specifically, while every other renderer path in the project - `vello_cpu`
and `vello_hybrid` on Vulkan/Metal/DX12 - renders the same input correctly)

## Root cause

`vello_hybrid`'s image atlas is a single `wgpu::Texture` with `dimension: D2` and
`depth_or_array_layers` equal to however many atlas "layers" are currently allocated, sampled
through a `D2Array` texture view. wgpu's `gles` backend (`wgpu-hal`'s `src/gles/mod.rs`,
`get_info_from_desc`) has a documented heuristic: a `D2` texture descriptor with
`depth_or_array_layers == 1` is treated as a plain `GL_TEXTURE_2D`, not a `GL_TEXTURE_2D_ARRAY`,
*regardless of the requested view dimension* - this heuristic lives directly in `wgpu-hal`'s shared
`gles` module, which backs **both** native `Backends::GL` (desktop/Android OpenGL/GLES) **and**
WebGL (wasm) - there is no `target_arch` distinction anywhere in that upstream heuristic.

`vello_hybrid::render::wgpu::Programs::create_atlas_texture_array` (in
`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`) already knew about this heuristic and worked
around it - but only `#[cfg(target_arch = "wasm32")]`:

```rust
#[cfg(target_arch = "wasm32")]
let depth_or_array_layers = atlas_count.max(2);
#[cfg(not(target_arch = "wasm32"))]
let depth_or_array_layers = atlas_count;
```

The very first image ever uploaded allocates the atlas texture array with exactly 1 layer
(`atlas_count == 1`). On a native (non-wasm32) build selecting `wgpu::Backends::GL`, the
`#[cfg(not(target_arch = "wasm32"))]` branch keeps `depth_or_array_layers == 1`, so the real GLES
driver silently creates a plain `GL_TEXTURE_2D` instead of the `GL_TEXTURE_2D_ARRAY` the rest of the
renderer's shaders and bind group assume - `queue.write_texture`'s upload (targeting array layer 0)
and the fragment shader's array-layer-indexed sample both operate against a texture object that
GL/GLES itself doesn't consider layered, so the uploaded pixel data is never actually visible where
the shader samples from, and the image renders as fully transparent/blank. `Backends::VULKAN`,
`Backends::METAL` and `Backends::DX12` don't have this GLES-specific classification quirk at all,
so the same code renders correctly there - which is why this was invisible on the SwiftShader/
Vulkan-only differential sweep and would be invisible in this project's own `hybrid`-suffixed test
suite (which requests a default/Vulkan-preferring adapter, never `Backends::GL` explicitly, and has
no native-desktop-GL-specific test variant - only a WASM/WebGL one, which is a structurally
different code path in `render/webgl/mod.rs` that doesn't go through this function at all).

## Introduced by

b7aac65f "[vello_hybrid]: Multiple image atlases support (#1252)" (2025-10-02) - the commit that
introduced the multi-atlas 2D texture array and this exact `depth_or_array_layers` workaround,
gated `#[cfg(target_arch = "wasm32")]` from the start (found via git log -S on the wasm32-gated
comment text; the gate survived unchanged, only reformatted, through two later touch-up commits,
f8303ee8 and 53df8b97).

## The fix

Removed the `#[cfg(target_arch = "wasm32")]` / `#[cfg(not(target_arch = "wasm32"))]` split in
`create_atlas_texture_array` and always allocate `atlas_count.max(2)` physical layers, regardless of
target arch - matching the fact that the underlying `wgpu-hal` `gles` backend quirk this works
around is shared by native `Backends::GL` and WebGL alike, not wasm-specific. The extra spare layer
is harmless on every other backend (a few extra bytes of GPU memory only ever allocated once, per
the existing "reuse a spare layer" logic already present in `maybe_resize_atlas_texture_array`,
which already handles a physically-larger-than-logically-needed atlas correctly).

- Both the 22-scene differential sweep and the minimal single-image repro confirm the fix: the two
  previously-blank image scenes (`image`, `multi_image`) return to the same small numeric-divergence
  range already seen on every other scene/backend combination (see `output.txt`), and the minimal
  repro's 100%-pixel-mismatch failure becomes a clean 0%-mismatch pass, reproduced 3/3 times.
- Project test suite (`cargo test -p vello_common -p vello_cpu -p vello_hybrid -p vello_encoding
  --release --lib`, this checkout has no `git-lfs`-fetched snapshot PNGs so the LFS-dependent
  `vello_sparse_tests`/`vello_tests` snapshot suites were not run - see README.md's documented
  environment gap): **394 passed, 0 failed, 1 ignored, before -> 394 passed, 0 failed, 1 ignored,
  after** (identical counts; no test's outcome changed by this fix).

## What's not done / open questions

- This session's differential harness needed candidate 065's already-recorded `compare_gpu_cpu`
  self-comparison fix to be usable at all; that fix was re-derived and applied locally to this
  session's own worktree purely as an enabling step (matching what candidate 065 itself did and
  reverted) - it is **not** part of this candidate's own `patch.diff`, and this candidate's own
  RESULT/repro do not depend on it (the minimal repro renders both sides directly and compares them
  itself, without touching `vello_tests`).
- `sparse_strips/vello_hybrid/src/render/webgl/mod.rs` (the separate, wasm-only `WebGlRenderer` that
  talks to `web_sys::WebGl2RenderingContext` directly rather than through `wgpu`) was not audited for
  the same class of bug - it's a structurally different implementation (no `wgpu-hal` involved) and
  its own `#[cfg(target_arch = "wasm32")]` gates are legitimate there (the whole file only compiles
  for wasm), but it wasn't checked for an analogous "single-layer texture array" assumption.
  `sparse_strips/vello_sparse_tests/tests/image_atlas.rs` (a `wasm_bindgen_test`-only suite covering
  `WebGlRenderer`'s atlas behavior) could not be run natively in this sandbox to cross-check it.
  Given both `render/wgpu/mod.rs`'s own `maybe_resize_atlas_texture_array` (untouched by this fix)
  and `render/webgl/mod.rs` independently reference the identical wgpu-hal GLES heuristic in
  comments, it's worth a follow-up read.
- This project has an **unmerged, in-progress branch** (`origin/laurenz/2d_array`, commit 31e733fc
  "vello_hybrid: Replace the 2D texture array with individual atlas textures") that replaces this
  whole texture-array atlas design with individual per-image atlas textures - which would likely
  sidestep this exact bug class structurally (no `D2Array` texture, no GLES layer-count heuristic to
  work around at all). This wasn't investigated further (out of scope: this candidate targets the
  current `main` branch, not an open, unmerged, unreviewed branch), but is worth noting for whoever
  triages this: the fix here may become moot if/when that branch merges, or may be exactly the kind
  of case that branch's own author already had in mind.
- Whether the residual, much-smaller (~0.06/255 mean, single-pixel edge outliers up to 200/255 on
  one diagonal-image-edge pixel) divergence still present on the GL backend *after* this fix (see
  `output.txt`, scene 9) is itself a separate, smaller bug (e.g. atlas edge-padding/bilinear-sampling
  boundary behavior differing slightly between backends) or just ordinary floating-point noise
  wasn't investigated - it's within the same small-scale range already seen on the Vulkan backend
  and on every non-image scene, so it doesn't look like the same catastrophic-blank-image bug class.
