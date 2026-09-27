# VELLO_HYBRID_GLYPH_ATLAS_TOOLARGE_EXPECT_PANIC

`projects/vello.md`'s risk map flags `vello_hybrid`'s wgpu backend as "comparatively less
explored than `vello_cpu`/glifo" and names its glyph-atlas-caching integration as an area not yet
swept for the same `.expect()`-on-a-legitimate-`Err` pattern already confirmed elsewhere in this
backend (candidates 026, 049, 055). `Renderer::render()`'s (`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`)
`#[cfg(feature = "text")]` block calls `resources.before_render(self, |renderer, glyph_renderer,
atlas_count, atlas_config, atlas_id| { renderer.render_to_atlas(...).expect("Failed to render
glyphs to atlas") }, ...)` - `render_to_atlas` returns `Result<(), RenderError>`, and this call
site discards that `Result` via `.expect()`, matching the same shape of bug as 026's
`image_cache.allocate(...).unwrap()`. The identical pattern also exists, byte-for-byte, in the
WebGL backend's own `render()`.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`, `src/render/webgl/mod.rs`, `src/text.rs`, `src/render/common.rs` (`LayersConfig::required_intermediate_texture_size`), `glifo/src/glyph.rs` and `glifo/src/renderer.rs` (glyph-atlas-cache insertion/replay) | ~50 min | Found the `.expect("Failed to render glyphs to atlas")` call sites (both backends) and traced when `render_to_atlas` legitimately returns `Err(RenderError::IntermediateTexture(IntermediateTextureError::TooLarge))`: `LayersConfig::required_intermediate_texture_size` rejects any layer bigger than the caller's own `max_texture_size`, whose own doc explicitly documents setting it small as an intended, supported choice ("Setting this to smaller ... to explicitly reject too large layers to limit peak memory consumption") |
| Manual boundary-value construction + real end-to-end repro, attempt 1: call the public, `#[doc(hidden)]` `Renderer::render_to_atlas` directly with a hand-built `Scene` and a mismatched `image_atlas_config.atlas_size` vs `layers_config.max_texture_size` | Release build, real SwiftShader Vulkan adapter | Reproduced a `TooLarge` `Err` from `render_to_atlas` itself, but on reflection this doesn't demonstrate the actual bug: `render_to_atlas` already correctly returns `Err` in that scenario (nothing panics unless the caller chooses to `.expect()` it themselves, which isn't what the library does internally) - discarded as not a genuine repro of *this* bug and not carried into `repro/` |
| Manual boundary-value construction + real end-to-end repro, attempt 2 (final): real `vello_hybrid::Scene` + real `glifo`/`vello_common` `FontData`/`Glyph` types + the vendored Chromium COLRv1 conformance test font (`examples/assets/colr_test_glyphs/test_glyphs-glyf_colr_1.ttf`, already used by this repo's own `glyphs_colr_test_glyphs` test) + a real `Renderer::new_with` with a small, fully-documented-as-supported `LayersConfig::max_texture_size` (8x8px) + `Scene::glyph_run().fill_glyphs()` (real, public glyph-rendering API) + real `Renderer::render()`, driven by a real `wgpu::Device`/`Queue` (SwiftShader software Vulkan adapter) | Release build | **Reproduced**: rendering glyph id 84 of that font at 100px panics inside the crate's own `Renderer::render()` at `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs:292`, `"Failed to render glyphs to atlas: IntermediateTexture(TooLarge { width: 52, height: 76, max_width: 8, max_height: 8 })"`. Reproduces 3/3 runs (release build, deterministic - no seed/layout dependency, same glyph/font/config every time). See `output.txt` |

Two escalating approaches were needed: the first (calling `render_to_atlas` directly) initially
looked like it reproduced the bug but on closer inspection only proved that function's *own*,
already-correct `Result`-returning behavior - not the library's own internal panic. The second
(driving the bug through the real glyph-rendering entry point, `Scene::glyph_run().fill_glyphs()`)
is what actually reaches the buggy `.expect()` inside `Renderer::render()`.

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs, release build, real (software) Vulkan
adapter via SwiftShader (`VK_ICD_FILENAMES` pointed at a bundled Playwright/Chromium SwiftShader
ICD - no real GPU used or required).

**Fully reproducible**: **Yes**. This is a single-participant finding (the library's own public
`Renderer::render()`/`Scene::glyph_run()` API reacting to its own caller's font/config data, not
one participant reacting to a peer's wire input) - `CONFIRMATION.md` section 11's "real second
participant" bar doesn't apply here, the same reasoning as candidates 026/049/055's own "Fully
reproducible" notes. `repro/src/main.rs` drives the actual, real, public
`Renderer::new_with`/`Scene::glyph_run().fill_glyphs()`/`Renderer::render()` call sequence a real
caller would make, through a real `wgpu::Device`/`Queue`, with a real vendored COLRv1 test font -
nothing here is hand-constructed to stand in for another participant's behavior.

## Class

crash

## Root cause

`Renderer::render()` (`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`, and identically in
`src/render/webgl/mod.rs`) renders any pending glyph-atlas-cache commands via
`Resources::before_render` (`src/text.rs`) before rendering the main scene. The closure passed to
`before_render` calls the real, already-correct `Renderer::render_to_atlas` and discards its
`Result` with `.expect("Failed to render glyphs to atlas")`.

`render_to_atlas` legitimately returns `Err(RenderError::IntermediateTexture(IntermediateTextureError::TooLarge))`
whenever the atlas-layer scene it's asked to render needs an intermediate GPU layer bigger than
`LayersConfig::max_texture_size` (`src/render/common.rs`'s `required_intermediate_texture_size`).
`max_texture_size` is a fully public, documented `LayersConfig` field whose own doc explicitly
frames a small value as an intended choice ("Setting this to smaller ... to explicitly reject too
large layers to limit peak memory consumption"). Any caller who takes that documentation at face
value and renders a color/COLR glyph whose nested paint layers exceed that bound gets a hard
process panic instead of the `Result::Err` the crate's own public API signature (`Renderer::render`
returns `Result<(), RenderError>`) promises they can handle.

This is reached through the crate's real glyph-atlas caching path: `Scene::glyph_run(...).atlas_cache(true).fill_glyphs(...)`
(a cache miss) synchronously allocates the glyph an atlas slot and queues a pending
rasterization command (`glifo`'s `GlyphAtlas::insert`); the very next `Renderer::render()` call's
`before_render` replays that pending command into the glyph-atlas's own scratch `Scene` (sized to
`image_atlas_config.atlas_size`, per `GlyphAtlasResources::with_config` in `text.rs`) via
`render_to_atlas` - and if that replay needs a bigger intermediate layer than `max_texture_size`,
the panic fires, from ordinary text rendering, with no scene-size or atlas-size misconfiguration
required at all (glyph id 84 of the vendored test font needs only a 52x76px layer at 100px font
size - `max_texture_size` merely needs to be set below that, which the crate's own docs invite).

The identical `.expect(...)` pattern (same message, same shape) also exists in the WebGl backend's
`WebGlRenderer::render()` - not separately exercised with its own `wgpu`-free repro this session
(WebGL needs a browser/`wasm32` target this environment can't drive), but read and root-caused as
the same bug and fixed identically (see `patch.diff`), since `Resources::before_render`'s changed
signature is shared by both backends and both call sites needed updating to keep the workspace
compiling.

## Introduced by

46cc30b7 "Migrate vello sparse strips renderers to use glifo (#1562)" (2026-04-14) - this commit
introduced glifo-based glyph rendering into `vello_hybrid` and, with it, both `.expect("Failed to
render glyphs to atlas")` call sites (wgpu and WebGL backends) in their current form. Verified via
`git log --reverse -S'"Failed to render glyphs to atlas"'` restricted to `HEAD`'s own ancestry (an
unrelated, non-ancestor fork commit with the same string was excluded after checking
`git merge-base --is-ancestor`).

## The fix

`Resources::before_render` (`src/text.rs`) now takes a `render_to_atlas` closure that returns
`Result<(), RenderError>` instead of `()`, and itself returns `Result<(), RenderError>`: it still
replays every pending atlas page (so one rejected/oversized page doesn't leave other, otherwise-fine
pages permanently stuck un-replayed), but propagates the *first* error it saw instead of silently
discarding it. Both call sites (`src/render/wgpu/mod.rs`'s and `src/render/webgl/mod.rs`'s
`render()`) now pass through `render_to_atlas`'s real `Result` instead of `.expect()`-ing it, and
propagate `before_render`'s own result with `?` - turning the panic into the same
`RenderError::IntermediateTexture(IntermediateTextureError::TooLarge)` a caller already knows how
to handle from the main scene's own, already-correct error path.

Verified: `repro/` no longer panics against the patched tree - `Renderer::render()` returns
`Err("Intermediate texture allocation 52x76 exceeds maximum 8x8")` instead (3/3 runs, see
`output.txt`'s "AFTER FIX" section).

- Project test suite (`cargo test -p vello_hybrid --all-features --release`): 122 passed / 0 failed
  before the fix, 122 passed / 0 failed after (1 additional doctest ignored, unaffected either
  way). `cargo check --workspace --all-features` also succeeds after the fix (confirms the WebGL
  backend and every other consumer of `Resources::before_render` still compile against the changed
  signature).
- `cargo test -p vello_sparse_tests --release` (broader integration/snapshot-image test suite):
  identical result before and after the fix (89 passed / 3538 failed / 1230 ignored, in 41s) - the
  3538 failures are pre-existing and environmental, not caused by this change: this worktree's PNG
  snapshot references are `git-lfs`-tracked and are plain-text LFS pointer stubs (confirmed via
  `file`) rather than real images in this `git-lfs`-less checkout, so every snapshot-image
  comparison in that suite fails regardless of any code change.

## What's not done / open questions

- The WebGL backend's identical `.expect(...)` (`src/render/webgl/mod.rs`) was fixed alongside the
  wgpu backend (both share `Resources::before_render`'s signature, so both needed updating to keep
  the workspace compiling) but was not independently re-verified with its own standalone WebGL
  repro this session - WebGL needs a browser/`wasm32` target, which this environment can't drive.
  The fix itself is the same three-line change applied identically to both backends' call sites,
  and `cargo check --workspace --all-features` confirms it compiles.
- Not checked whether any other `vello_hybrid` internal caller of `Resources::before_render`
  outside these two backends exists that would also need updating (a grep across the workspace
  after the fix found none besides the two already fixed).
- This is the same general class of bug as 026 (`.unwrap()` on `AtlasError` in `upload_image`) and
  049 (`AtlasConfig` cross-field invariant) - a `Result`-returning internal API whose caller
  discards the `Result` via `.expect()`/`.unwrap()` instead of propagating it. `src/lib.rs`'s own
  `RenderError` doc already carries a `// TODO: Consider expanding RenderError to replace some
  .unwrap and .expect.` comment acknowledging this is a known, recurring gap - grepping
  `vello_hybrid/src` for `.expect(` and `.unwrap()` on anything returning `Result`/`Option` derived
  from scene/font/config data (not from an internal invariant already enforced elsewhere) is likely
  to turn up more of the same, beyond the specific call sites checked in 026/049/055/this candidate.
