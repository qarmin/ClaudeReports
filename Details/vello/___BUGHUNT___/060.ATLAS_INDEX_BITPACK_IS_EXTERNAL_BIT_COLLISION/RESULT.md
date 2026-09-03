# ATLAS_INDEX_BITPACK_IS_EXTERNAL_BIT_COLLISION

`sparse_strips/vello_hybrid/src/render/common.rs`'s `pack_image_params()` packs `atlas_index` into
an 8-bit field (bits 6-13) of the image-params `u32`, guarded only by
`debug_assert!(atlas_index <= 255, ...)` (compiled out in release). `AtlasConfig::max_atlases`
(`sparse_strips/vello_common/src/multi_atlas.rs`) is a public, `usize` field (default 8,
`auto_grow: true`) reachable through the public `RenderSettings.memory_settings.image_atlas_config`
field (`sparse_strips/vello_hybrid/src/scene.rs`). An `atlas_index` of exactly 256 (`0x100`) shifted
left by 6 equals `0x4000` = bit 14 - the same bit `pack_image_params` uses for `is_external`. Bit
14 is read back by the shader's `get_image_source_kind()` (`vello_sparse_shaders/shaders/helpers/image.wesl`),
so atlas 256 silently looks like an external texture to the fragment shader.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading: traced `pack_image_params`/`MemorySettings::normalize`/`get_image_source_kind`/`get_image_atlas_index` to confirm the exact bit arithmetic before building a repro | N/A (static) | Confirmed the collision is real arithmetic (`256 << 6 == 1 << 14`), not just a suspicion; also found `MemorySettings::normalize` already clamps `max_atlases` to the *device's* `max_texture_array_layers` - but never to the packed format's own 256-atlas ceiling, so a real device that reports more than 256 array layers (checked: SwiftShader reports 2048) leaves the bug fully reachable. |
| Real end-to-end release-mode run: real SwiftShader (software Vulkan) `wgpu::Device` requested with `required_limits: adapter.limits()` (2048 array layers), real `Renderer::new_with` with `AtlasConfig{max_atlases: 300, atlas_size: (1,1), auto_grow: true}`, 257 real `Renderer::upload_image()` calls (last one opaque red), real `Scene`/`renderer.render()` painting from the 257th image, real GPU readback | release build, `repro/`, 3 repeat runs | **Reproduced 3/3 runs**: rendered pixel `[0, 0, 0, 0]` instead of the uploaded `[255, 0, 0, 255]` - the atlas-backed image is silently sampled as if it were the (always-zero-initialized) external-texture placeholder. Fully deterministic, no seed/layout dependence. |
| Debug-assertions build of the identical real end-to-end path (`cargo run`, dev profile, no `--release`) | debug build, same `repro/`, 1 run | Panics exactly where predicted: `common.rs:663:5: atlas_index must be 0-255 (8 bits)`, at the moment `pack_image_params` is called with `atlas_index = 256` for the 257th upload's paint. Independently confirms the mechanism read from the code. |

Two real-tool escalations (a real end-to-end release run, then the same path under debug
assertions) both confirmed the same mechanism from the code reading - well within the soft cap of
3, no need for a third approach (miri/fuzzing don't fit this bug class: it's deterministic integer
bit-packing arithmetic on a value produced by real, reachable public configuration, not adversarial
parser input or `unsafe`/FFI).

## Status

**Confirmed**. Reproduces 3/3 runs (release build, SwiftShader software Vulkan adapter,
deterministic - not layout/seed-dependent). Independently corroborated once more under debug
assertions (the library's own `debug_assert!` fires at the exact predicted line).

**Fully reproducible**: **Yes**. `repro/` drives the real, public `vello_hybrid` API end-to-end: a
real `wgpu::Device`/`Queue` (SwiftShader), the real `Renderer::new_with`/`RenderSettings`/
`AtlasConfig` public configuration surface, 257 real `Renderer::upload_image()` calls that force the
real `MultiAtlasManager` to actually allocate 257 real atlases (not a hand-constructed `atlas_id`),
and a real `Scene`/`renderer.render()` call whose real GPU-readback pixel is the observed evidence.
This is a single-participant finding (one process configuring and driving its own renderer through
documented public API, not a caller/peer-input mismatch), so `CONFIRMATION.md` section 11's
two-participant bar does not apply.

## Class

logic / rendering-correctness (silent, undetected-in-release output corruption)

## Root cause

Three facts compound:

1. `AtlasConfig::max_atlases` (public, `usize`) has no upper bound anywhere in its own type or
   documentation.
2. `MemorySettings::normalize()` (called from both the `wgpu` and `webgl` `Renderer::new_with`)
   clamps `max_atlases` to the *device's* reported `max_texture_array_layers` - but real devices can
   (and, per this session's own SwiftShader adapter, do: 2048) report far more than 256. Nothing
   clamps `max_atlases` to the packed format's own 8-bit `atlas_index` ceiling (256 atlases,
   indices 0..=255).
3. `pack_image_params()` packs `atlas_index` into bits 6-13 with only a `debug_assert!` guarding the
   256-value ceiling; in a release build an `atlas_index` of 256 silently carries into bit 14
   (`is_external`), and the shader's `get_image_source_kind()` reads that same bit to decide whether
   to sample the atlas texture array or the externally-bound texture. With no external texture
   actually registered for this paint (`TextureBindings::new()` in the repro, matching an app that
   never uses external textures at all), the shader samples the always-zero-initialized 1x1
   placeholder external texture instead - a fully opaque, fully wrong image silently renders as
   transparent black. `get_image_atlas_index()`'s own `& 0xFFu` mask on readback also means even the
   *atlas index* half of this value wraps to 0, so if the shader ever did fall through to the atlas
   path it would sample the wrong atlas (atlas 0) instead of atlas 256, a second, narrower
   independent symptom of the same missing bound.

## Introduced by

The exact collision this candidate describes (`atlas_index` aliasing into the `is_external` bit)
was introduced by c95b228e "feat: support external textures in WebGL backend (#1792)" (2026-08-03),
which added the `is_external` bit at position 14 directly adjacent to the pre-existing 8-bit
`atlas_index` field without adding or updating any bound tying `max_atlases` to 256. Earlier
context, for completeness: the unbounded `max_atlases` field and the 8-bit `atlas_index` packing
scheme were both introduced together by b7aac65f "[`vello_hybrid`]: Multiple image atlases support
(#1252)" (2025-10-02); the device-limit clamp in `MemorySettings::normalize()` (which still doesn't
account for the packed format's own ceiling) was added later by f8303ee8 "vello_hybrid: Adjust atlas
config to actual device capabilities (#1568)" (2026-04-14). Before c95b228e, an `atlas_index` of 256
would have silently wrapped only the atlas-index readback to 0 (bug class 3 in "Root cause" above,
which still exists independently) rather than being reinterpreted as an external texture - the
specific "silently becomes external" symptom this candidate demonstrates dates to c95b228e.

## The fix

`MemorySettings::normalize()` now clamps `max_atlases` to `min(device's max_texture_array_layers,
256)` instead of just the device's own limit, via a new documented `MAX_PACKED_ATLAS_COUNT: usize =
256` constant defined next to `pack_image_params` itself (so the packed format's own ceiling is the
single source of truth, referenced from both the constant's doc comment and `normalize`). This is
the single shared clamp point for both the `wgpu` and `webgl` backends (both call
`MemorySettings::normalize` from their respective `Renderer::new_with`), so it protects both real
entry points, not just the one exercised by `repro/`. After the fix, a caller who requests
`max_atlases: 300` on a device that genuinely supports up to 2048 array layers gets `max_atlases`
silently (as before) clamped, but now to 256 instead of 2048 - so the allocator can never hand out
an `atlas_id` above 255, and the 257th allocation attempt honestly fails with the existing, already
well-formed `AtlasError::AtlasLimitReached` instead of succeeding and corrupting the render.

Verified: `repro/` re-run against the patched tree (same command, same build) no longer reproduces
the silent corruption - it now fails at the 257th `upload_image()` call with a clean
`AtlasLimitReached` (caught explicitly in `repro/`'s own `catch_unwind`, printed as "did not
reproduce"), confirming the bug is genuinely gone rather than the failure merely changing shape
(see `CONFIRMATION.md` section 6 - the new failure is a documented, typed, honest capacity error at
allocation time, not a differently-shaped crash masking the same underlying corruption).
Per `CONFIRMATION.md` section 7, also checked a second input in the same class: `128 <= max_atlases
<= 256` (i.e. staying at or under the true boundary) is unaffected by the clamp and continues to
allocate/render normally (covered by the existing `normalize_memory_settings_caps_atlas_count_to_backend_limit`
unit test, still passing unchanged).

Added one new unit test, `normalize_memory_settings_clamps_atlas_count_to_packed_format_limit`, next
to `normalize`'s existing test suite: a device reporting 2048 array layers with `max_atlases: 300`
requested must come out of `normalize()` clamped to exactly 256, not 2048.

- Project test suite:
  - `cargo test -p vello_hybrid --lib --release`: **before** 122 passed/0 failed/0 ignored ->
    **after** 123 passed/0 failed/0 ignored (122 pre-existing + 1 new regression test, all still
    passing).
  - `cargo test -p vello_common --lib --release`: **before** 159 passed/0 failed/1 ignored ->
    **after** 159 passed/0 failed/1 ignored (unchanged; the fix doesn't touch this crate).
  - Snapshot-based `vello_sparse_tests` suite was not run: this sandbox's checkout has unresolved
    `git-lfs` pointer stubs for the reference PNGs (see `___BUGHUNT___/README.md`), so it cannot
    meaningfully pass/fail here regardless of this patch.

## What's not done / open questions

- This fix closes the reachable channel through `Renderer::new_with`/`RenderSettings` (both `wgpu`
  and `webgl` backends, since both call the same shared `MemorySettings::normalize`). It does
  **not** add a bound inside `ImageCache::new_with_config`/`MultiAtlasManager::new` themselves - a
  caller who constructs an `ImageCache`/`MultiAtlasManager` directly with `AtlasConfig{max_atlases:
  300, ..}` (bypassing `Renderer::new_with` entirely) can still build an atlas manager whose
  `create_atlas()` will happily mint `AtlasId(256)` and beyond; that atlas simply isn't reachable
  from the shipped `wgpu`/`webgl` renderers today, so it doesn't reproduce the visual bug through
  any real entry point currently in this crate, but a future third backend/direct integration that
  skips `normalize()` would reintroduce the exact same class of bug. Whether to also enforce the
  256 bound at `MultiAtlasManager`/`ImageCache` construction time (defense in depth, independent of
  any specific renderer backend remembering to call `normalize()`) is a design decision left to the
  maintainers rather than bundled into this bounded fix.
- The narrower, independent symptom noted in "Root cause" point 3 (`get_image_atlas_index()`'s own
  `& 0xFFu` mask silently wrapping an out-of-range atlas index to 0, even without the `is_external`
  bit collision) was not separately exercised with its own repro this session - the fix above
  removes the only reachable way to produce an out-of-range `atlas_index` at all, so it's covered
  by the same patch, but it wasn't isolated as its own candidate/regression test.
- Not checked whether any other packed GPU-paint field in this same module (gradients, blurred
  rounded rects) has an analogous "public config value vs. fixed-width packed field" gap - this
  session only looked at the image/atlas path named in the candidate.
