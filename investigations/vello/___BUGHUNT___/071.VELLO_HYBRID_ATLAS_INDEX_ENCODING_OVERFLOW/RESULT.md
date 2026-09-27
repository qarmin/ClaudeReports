# VELLO_HYBRID_ATLAS_INDEX_ENCODING_OVERFLOW

General sweep of `vello_hybrid`'s wgpu backend for further device-limit/resource-exhaustion gaps,
following up on already-confirmed candidates 026 (`AtlasError` `.unwrap()`), 049 (atlas config
cross-field invariant panic), and 055 (a 4096px texture-dimension bound reused for an unrelated
resize-assert). Reading `sparse_strips/vello_hybrid/src/render/common.rs`'s `MemorySettings::normalize`
(the function whose entire job is clamping a caller-supplied `AtlasConfig` to what the backend can
actually support) alongside `pack_image_params`'s own doc comment ("`atlas_index`: stored in bits
6-13 (8 bits, supports up to 256 atlases)") showed `normalize` only clamps `max_atlases` to the
*device's own* reported `max_texture_array_layers` - never to the independent, fixed 256-atlas
ceiling the GPU wire encoding itself imposes. Real devices commonly report far more than 256 (wgpu's
own conservative default `Limits` value is exactly 256, but this sandbox's own SwiftShader adapter,
via `adapter.limits()`, reports 2048), so a caller who raises `AtlasConfig::max_atlases` above 256
(a legitimate thing to do - the type's own doc comment invites tuning it to "the capabilities of the
backend") and whose scene genuinely allocates more than 256 atlases ends up with `atlas_index`
values that overflow the packed field's 8 bits.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `sparse_strips/vello_hybrid/src/render/common.rs` (`MemorySettings::normalize`, `pack_image_params`, `DeviceLimits`) and `sparse_strips/vello_common/src/multi_atlas.rs` (`AtlasConfig`, `AtlasId`) against the "further unchecked device-limit assumptions" brief, ~25 min | Code reading | Found `normalize` clamps `max_atlases` only to `device_limits.max_texture_array_layers`, never to the 8-bit `atlas_index` field's own 256-atlas ceiling that `pack_image_params` documents and only checks via a `debug_assert!`. `AtlasId(pub u32)` and `AtlasConfig::max_atlases: usize` place no upper bound at the type level either. |
| Boundary-value construction: queried a real adapter's reported `max_texture_array_layers` via a small standalone probe (`wgpu::Instance` + SwiftShader Vulkan ICD) | Real (software) Vulkan adapter, `adapter.limits()` | Confirmed `max_texture_array_layers = 2048` for this sandbox's real adapter - directly demonstrating that `device_limits.max_texture_array_layers` alone is not a safe proxy for the 256-atlas encoding limit on real hardware, not just a hypothetical. |
| Debug-assertions build + real end-to-end repro: real `wgpu::Device` (requesting `adapter.limits()`), `Renderer::new_with` with `AtlasConfig::max_atlases = 2048`, 300x `Renderer::upload_image` with distinct 1x1 `Pixmap`s (atlas_size (1,1), so each forces its own new atlas), then `Scene::set_paint(Image{ImageSource::opaque_id(..)})` + `Renderer::render()` on the image with the highest atlas id | `cargo run` (dev/debug build), pristine tree, 5 total independent runs (3 during initial investigation directly against the checked-out worktree, plus 1 clean-rebuild run and 1 full `just clone`+`just reproduce-before` run against a genuinely separate fresh `git clone`) | **Reproduced 5/5**: `render()` panics inside `pack_image_params`'s `debug_assert!(atlas_index <= 255, "atlas_index must be 0-255 (8 bits)")` at `sparse_strips/vello_hybrid/src/render/common.rs:663`. Also re-ran with `count = 257` (the minimal over-the-limit case, atlas id 256 exactly, not just the arbitrary 299 used above) - reproduces identically, confirming this isn't specific to one particular atlas id. |
| Same repro, `cargo run --release` (release build, `debug_assertions` off) | Release build, pristine tree, count=300 | Does **not** panic - `render()` completes. This confirms the bug manifests differently by build: a debug build's `debug_assert!` turns it into a clean panic, while a release build silently packs the overflowed `atlas_index` bits into `image_params`'s adjacent fields (bit 14 is `is_external`; `atlas_index=299` (`0x12B`) shifted left 6 sets bit 14, so the GPU shader would misinterpret the image as `is_external == true` and sample garbage/the placeholder external-texture slot instead of the atlas - real rendering corruption, not just a masked panic). |

## Status

**Confirmed**

Reproduces 5/5 runs (debug build, pristine tree) across two independent environments (the checked-
out project worktree, and a genuinely separate `git clone` at the recorded `base_commit`), including
one run at the minimal boundary value (atlas id exactly 256) and one at a larger value (atlas id
299) - both fail identically. Release-build behavior (silent corruption, no panic) was separately
observed once and is consistent with reading `pack_image_params`'s packing arithmetic directly; it
is reported as a build-dependent variant of the same root cause, not as an independently-repeated
finding.

**Fully reproducible**: **Yes**. `repro/src/bin/before.rs` drives `vello_hybrid`'s own real public
API end-to-end against a real `wgpu::Device`/`Queue` obtained from a real (software) Vulkan adapter
(SwiftShader) - `Renderer::new_with`, `Renderer::upload_image`, `Scene::set_paint`/`fill_rect`, and
`Renderer::render()` are all exercised for real, with no hand-constructed internal state standing in
for anything. This is a single-participant finding (a caller misusing/exceeding a documented,
tunable config knob interacting with the crate's own internal wire-format limit) - there is no
second peer/caller whose behavior needs to be faked; requesting `adapter.limits()` and setting
`AtlasConfig::max_atlases` above the tiny default of 8 is a completely ordinary thing for a native
application to do, not an adversarial input.

## Class

logic (silent data corruption in release builds) / crash (debug_assert panic in debug builds)

## Root cause

`sparse_strips/vello_hybrid/src/render/common.rs`'s `pack_image_params` packs an image's atlas
index into an 8-bit field (bits 6-13 of `image_params`, "supports up to 256 atlases" per its own
doc comment) and only guards this with a `debug_assert!`, not a real check. The one place meant to
prevent an out-of-range value from ever reaching that function - `MemorySettings::normalize`, called
from `Renderer::new_with` - clamps the caller-supplied `AtlasConfig::max_atlases` only to
`device_limits.max_texture_array_layers` (whatever the real device happens to report, which is
commonly well above 256: wgpu's own conservative default `Limits::max_texture_array_layers` is
exactly 256, but real/software adapters routinely report more, e.g. 2048 in this sandbox).
`AtlasConfig::max_atlases: usize` and `AtlasId(pub u32)` place no additional bound anywhere else in
the atlas-allocation code path (`vello_common::multi_atlas::MultiAtlasManager::create_atlas`
happily creates atlas index 256, 257, ... up to whatever `max_atlases` allows). So once a caller
raises `max_atlases` above 256 (matching what their device reports) and the scene genuinely
allocates more than 256 image atlases, the 257th+ atlas's `atlas_id` overflows `pack_image_params`'s
8-bit field: in a debug build this trips the `debug_assert!` and panics; in a release build it
silently corrupts the packed `image_params` u32 (in particular, it can flip bit 14, `is_external`,
causing the GPU shader to sample the wrong texture entirely).

## Introduced by

f8303ee8 "vello_hybrid: Adjust atlas config to actual device capabilities (#1568)" - this is the
commit that introduced `MemorySettings::normalize` (and its `device_limits.max_texture_array_layers`
clamp) in the first place. The 8-bit `atlas_index` encoding itself is older (introduced by b7aac65f
"[`vello_hybrid`]: Multiple image atlases support (#1252)"), so the *possibility* of this overflow
existed since b7aac65f, but there was no clamping function at all until f8303ee8 - and f8303ee8 (and
the later c3343862 "vello_hybrid: A complete rewrite of the core logic (#1759)", which carried the
same `normalize` logic forward unchanged) is the commit that added a clamp covering only the
device's own limit while missing the encoding's own independent 256-atlas ceiling.

## The fix

Added a `MAX_ATLASES_SUPPORTED_BY_ENCODING: usize = 256` constant next to `pack_image_params` and
changed `MemorySettings::normalize` to clamp `max_atlases` to `device_limits.max_texture_array_layers
.min(MAX_ATLASES_SUPPORTED_BY_ENCODING)` instead of the device limit alone. Also switched
`pack_image_params`'s existing `debug_assert!` to compare against the same named constant (purely
for consistency/documentation - it was already correct, just using a bare `255` literal). This is
the same fix shape as candidate 049 (clamping a config value against a real invariant at the single
place meant to enforce it) and keeps the change minimal and localized to the one function whose job
is exactly this kind of clamping.

Verified against two inputs in the same class, not just the one that first triggered it (per
CONFIRMATION.md section 7): re-ran the repro with `count = 256` (the new correct boundary - the
maximum number of atlases the fix now allows) against the patched tree and confirmed `render()`
succeeds for atlas id 255 without panicking (see `repro/src/bin/after.rs` / `output.txt`); separately
confirmed (during investigation, not part of the shipped repro) that the fixed-boundary case
(`count = 257`, one over the new cap) instead raises the already-independently-confirmed candidate
026 `AtlasError`-unwrap panic in `upload_image` - a different, already-tracked issue, not a
regression introduced by this fix (this fix's scope is only "never let `atlas_index` overflow its
8-bit field", not "how atlas exhaustion itself is surfaced to callers").

- Project test suite (`cargo test -p vello_hybrid --lib`): 122 passed, 0 failed both before and
  after the patch (no regressions; the existing `normalize`-related unit tests in
  `render/common.rs` all use `max_texture_array_layers` values below 256, so none of them exercised
  the new clamp's boundary - see "What's not done" below).

## What's not done / open questions

- No new unit test was added inside `sparse_strips/vello_hybrid/src/render/common.rs`'s own
  `#[cfg(test)]` module for the new 256-atlas clamp (e.g. `device_limits(2048, ...)` ->
  `max_atlases` clamped to 256) - the existing test suite doesn't cover this boundary at all
  (confirmed: all existing `normalize_memory_settings_*` tests use small `max_texture_array_layers`
  values), so a maintainer taking this patch should add one alongside it.
- The release-mode "silent corruption" claim (`is_external` bit getting flipped) is derived from
  reading `pack_image_params`'s packing arithmetic directly (`atlas_index=299` = `0x12B`, shifted
  left 6 sets bit 14) plus observing that `render()` completes without panicking in `--release`,
  not from actually reading back the corrupted `image_params` value from GPU memory or observing a
  visibly wrong rendered pixel - that would need either mapping the GPU-side encoded-paints texture
  back to the CPU or a headless image-comparison pipeline, judged out of scope for a single-candidate
  fix-verification pass this session.
- Whether any other packed GPU structure in `render/common.rs` (e.g. `GpuEncodedImage`'s other
  packed fields, or the gradient-texture-width packing in `pack_texture_width_and_extend_mode`)
  has a similar "device limit clamped, but not the wire-format's own narrower limit" gap was not
  swept this session - `pack_image_params` was the one directly motivated by this candidate's
  atlas-config reading, not an exhaustive audit of every `pack_*` function in the file.
