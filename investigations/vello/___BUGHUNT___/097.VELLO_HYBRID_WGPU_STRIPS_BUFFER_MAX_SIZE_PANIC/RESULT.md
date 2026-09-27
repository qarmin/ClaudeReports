# VELLO_HYBRID_WGPU_STRIPS_BUFFER_MAX_SIZE_PANIC

`vello_hybrid`'s wgpu backend has already yielded several device-limit/resource-exhaustion bugs
(026, 049, 055, 090), all in the atlas/gradient/paint/filter texture-resize code, each guarded by
at least an `assert!`/`.unwrap()` against *some* (sometimes wrong) bound. This candidate looked for
an *unchecked* case elsewhere in the same backend's encoding/dispatch path and found one: the
per-frame `GpuStrip` vertex buffer (`create_strips_buffer`/`upload_strip_pair` in
`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`) is created directly from the real strip count
for a draw pass with **no size check at all** - not even an `assert!` - before calling
`Device::create_buffer`. wgpu's own default "uncaptured error" handler turns a resulting
buffer-too-large validation error into a hard process panic.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `vello_hybrid`'s wgpu-backend buffer/texture creation call sites (`create_strips_buffer`, `create_buffer_init` for blend/copy/clear instances, `create_intermediate_texture`) against `Device::limits()` | Code reading, ~40 min | `create_strips_buffer` (label "Strips Buffer") is the only one with a **plausibly reachable** unchecked size: it's created from `opaque_len + alpha_len` (the frame's real `GpuStrip` count times 24 bytes) with no clamp/check against `device.limits().max_buffer_size` anywhere in the call chain (`upload_strip_pair` -> `strip_pass_inner` -> `Backend::opaque_draw_pass`/`draw_pass`). The intermediate-texture and atlas/gradient/paint/filter-texture paths already have *some* guard (assert or `Result`), even if 055/090 found those guards used the wrong bound. |
| Probed real `wgpu::Limits` via a standalone program against this sandbox's real (software) Vulkan adapter (Google SwiftShader) | One-off probe, ~5 min | `adapter.limits().max_buffer_size` = 1 GiB on this adapter, but a device created with `wgpu::Limits::default()` (the WebGPU-spec-mandated minimum, and what a typical embedder requests unless it deliberately opts into higher limits) has `max_buffer_size` = 268,435,456 (256 MiB) - confirmed empirically that `Device::create_buffer` with a size 1 byte over this limit panics via wgpu's default uncaptured-error handler (`wgpu_core.rs:1614`), not a graceful `Result`. |
| Read `sparse_strips/vello_hybrid/src/rect.rs`'s "rectangle fast path" (`split_rect`) and `draw.rs`'s `GpuStrip::from_rect` to find the cheapest amplification from public-API draw calls to strip count | Code reading + existing unit test (`splitter_keeps_large_aligned_rect_as_single_main_rect`), ~20 min | Confirmed any axis-aligned, pixel-boundary-aligned `Scene::fill_rect` produces **exactly one** `GpuStrip` regardless of the rect's size - so N such draws in a single frame produce N strips (24 bytes each), independent of rect dimensions. |
| Manual boundary-value construction + real end-to-end repro: a real `vello_hybrid::Scene` (public API) filled with millions of distinct-colored, non-overlapping 1x1 `fill_rect` calls (distinct color per rect to prevent strip-generation from coalescing adjacent same-paint strips into fewer, wider ones), rendered through the real, public `vello_hybrid::Renderer::render` against a real `wgpu::Device`/`Queue` (SwiftShader, `wgpu::Limits::default()`) | Release build | **Reproduced**: 4000x3000 = 12,000,000 rects (288,000,000 bytes, over the 256 MiB `max_buffer_size`) panics at `Device::create_buffer, label = 'Strips Buffer'`. 3000x3000 = 9,000,000 rects (216,000,000 bytes, under the limit) renders successfully. Reproduces 3/3 runs, deterministic - see `output.txt`. |

Three escalating approaches (code reading against the already-known bug class from 026/049/055/090,
a real-adapter probe to establish the exact numeric boundary, then a real end-to-end boundary-value
repro) were sufficient for a clean, deterministic reproduction on real (software) GPU hardware - no
fuzzing/miri/ASAN needed, since this is a plain arithmetic-capacity-vs-device-limit question, not a
memory-safety/UB one.

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs, release build, real Vulkan adapter
(SwiftShader), `wgpu::Limits::default()`. See `output.txt` for the full before/after capture plus
boundary checks (9,000,000 rects succeeds both before and after; 12,000,000 rects with
`HIGH_LIMITS=1` - i.e. a device whose real `max_buffer_size` is 1 GiB - succeeds both before and
after, showing the panic is specifically about the *configured device's* limit, not an unconditional
rejection).

**Fully reproducible**: **Yes**. This is a single-participant finding (the library's own public
`Renderer::render`/`Scene` API reacting to its own caller's scene data - a scene with many small
draws - not one participant reacting to a peer's wire input), the same reasoning candidates 026,
055, and 090 already used for this backend. `repro/src/main.rs` drives the actual, real, public
`Scene::set_paint`/`Scene::fill_rect`/`Renderer::render` call sequence a real caller would make
(e.g. an embedder rendering a densely-packed grid/mosaic UI, or many independently-styled tiny
elements from generated content), through a real `wgpu::Device`/`Queue` obtained from a real
(software) Vulkan adapter - nothing about the scene data or the GPU pipeline execution is faked or
hand-constructed to stand in for another participant.

## Class

crash

## Root cause

`Programs::upload_strip_pair` (`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`) computes
`total_len = opaque_len + alpha_len` (the byte size of the current draw pass's `GpuStrip` data,
24 bytes per strip) and immediately calls `Self::create_strips_buffer(device, total_len)`, which is
a bare `device.create_buffer(&wgpu::BufferDescriptor { size: required_strips_size, ... })` - with
**no check anywhere in the call chain** against `device.limits().max_buffer_size`. This differs from
every other device-limit bug already found in this backend (026's `.unwrap()` on `AtlasError`, 049's
unchecked atlas-count loop, 055/090's `assert!` against an artificially-capped texture-height
constant): here there isn't even a wrong/miscalibrated guard - the buffer is created directly from
untrusted-scale scene data with no validation step at all. wgpu's own default "uncaptured error"
handler (active whenever the application hasn't installed `Device::on_uncaptured_error` or pushed an
error scope, which `vello_hybrid`'s own examples/tests do not) turns the resulting
`Device::create_buffer` validation failure into an unconditional `panic!`.

Every axis-aligned, pixel-boundary-aligned `Scene::fill_rect` call reaches
`sparse_strips/vello_hybrid/src/rect.rs`'s "rectangle fast path" and produces exactly one
`GpuStrip` (`GpuStrip::from_rect` in `draw.rs`) regardless of the rect's size (confirmed by the
existing unit test `splitter_keeps_large_aligned_rect_as_single_main_rect`). A single frame's root
opaque draw pass batches all such strips from the whole scene into one `Backend::opaque_draw_pass`
call, so a scene with more than 11,184,810 such rects (`268,435,456 / 24`, the exact byte-level
boundary for the WebGPU-spec-default 256 MiB `max_buffer_size`) reliably crosses this unchecked
limit and panics - a scale that's large but far more directly reachable via ordinary content (a
densely-packed grid, mosaic, or spreadsheet-like UI with many small independently-styled cells) than
055's 4200-gradient or 090's 5.6-million-filter-layer triggers, since it requires only a single,
extremely common draw primitive (`fill_rect`) repeated, not a specific paint/filter feature.

## Introduced by

f57a1d1b "`vello_hybrid` implementation (#831)" (2025-03-13) - the original `vello_hybrid`
implementation PR already created the strips buffer directly from the frame's data with a
`label: Some("Strips Buffer")` and no size validation (confirmed via `git log -S "Strips Buffer"
--all -- .`, which returns only this one commit). The code has since been reorganized (9d1208b9
"vello_hybrid: Move backends into their own folders (#1797)" moved it into its current
`render/wgpu/mod.rs` location without changing this behavior - confirmed via `git log -S
"create_strips_buffer"`, which also returns only that one, purely structural commit), but the
missing size check itself traces back to the original implementation.

## The fix

Added a `max_buffer_size: u64` field to `Programs`, populated once in `Programs::new` from
`device.limits().max_buffer_size` (the real, current device limit - not a hardcoded constant).
`Programs::upload_strip_pair` now checks `total_len` against it before calling
`create_strips_buffer`, returning a new `RenderError::StripsBufferTooLarge { required, max }`
variant instead of proceeding to the unchecked `Device::create_buffer` call. This error is
propagated up through the render pipeline:

- `RendererContext::strip_pass_inner` (wgpu backend) now returns `Result<(), RenderError>`,
  propagating `upload_strip_pair`'s error via `?`.
- The `Backend` trait's `opaque_draw_pass`/`draw_pass` methods (`sparse_strips/vello_hybrid/src/
  schedule/execute.rs`) now return `Result<(), RenderError>` instead of `()`; `Schedule::execute`/
  `Rounds::execute` propagate this at their 3 call sites (the root opaque pass, per-layer draw
  passes, and the root draw pass) via `?`.
- `Renderer::render_scene`'s call to `crate::schedule::execute(...)` (already inside a function
  returning `Result<(), RenderError>`) now uses `?` instead of discarding the result.
- The WebGL backend's `Backend` impl (`render/webgl/mod.rs`) was updated only to match the new
  trait signature (wraps its existing, unchanged body in `Ok(())`) and its own `render_scene`'s
  `crate::schedule::execute(...)` call now also uses `?` - its own `strip_pass_inner`/buffer
  creation was **not** modified; see "What's not done" below.
- The test-only `Recorder` (`schedule/execute.rs`'s `#[cfg(test)] mod tests`) and
  `ScheduledCase::execute` (`schedule/test_support.rs`) were updated to match the new fallible
  signatures (`Ok(())` / `.unwrap()` respectively) so the existing scheduler unit tests keep
  compiling and passing unchanged.

Verified against the original triggering input (12,000,000 rects: was a wgpu-internal validation
panic on `Device::create_buffer`, now returns `Err(RenderError::StripsBufferTooLarge { required:
288000000, max: 268435456 })` gracefully - `output.txt` section 3) **and** per `CONFIRMATION.md`
section 7, varied the input within the same bug class:
- 9,000,000 rects (under the limit): renders successfully both before and after the fix (`output.txt`
  sections 2 and 4) - the fix does not narrow real capacity.
- 12,000,000 rects with `HIGH_LIMITS=1` (requests the adapter's own real `max_buffer_size`, 1 GiB on
  this SwiftShader adapter, instead of the WebGPU-spec 256 MiB default): renders successfully both
  before and after the fix (`output.txt` section 5) - proving the fix's bound genuinely tracks the
  configured device's real capacity rather than unconditionally rejecting any large scene.

- Project test suite, before vs. after (identical in both cases - no regression; none of the
  existing tests exercised a strips buffer anywhere near this size):
  - `cargo test -p vello_hybrid --all-features --lib`: 122 passed; 0 failed -> 122 passed; 0 failed.
  - `cargo check -p vello_hybrid --all-features` (covers both the wgpu and webgl `Backend` impls):
    0 errors, 0 warnings, both before and after.

## What's not done / open questions

- The WebGL backend (`sparse_strips/vello_hybrid/src/render/webgl/mod.rs`) has its own, separate
  `strip_pass_inner`/strip-buffer creation with (as far as this session determined by inspection,
  not an independent repro) the same missing-size-check shape - deliberately left unfixed to keep
  scope to the wgpu (native) backend, matching the scope decision candidates 055/090 already made
  for this backend pair. Its `Backend` impl was only updated to satisfy the new trait signature.
- This session found and fixed only the strips buffer specifically. The sibling
  `create_buffer_init` calls for blend instances, copy instances, and clear instances
  (`blend_pass_inner`/`clear_pass_inner` in the same file) have the identical missing-check shape
  in principle, but were not independently confirmed with a working repro this session - they
  require amplifying via layer blend/clip/filter operations rather than a single flat `fill_rect`
  loop, which this session judged less directly reachable at a comparable scale and did not pursue
  further within the session's scope. Worth a follow-up candidate.
- Did not check whether `vello_hybrid`'s intermediate-texture creation path
  (`create_intermediate_texture`) has an equivalent gap for the texture *count* (as opposed to a
  single buffer's byte size) when `LayersConfig::max_textures` is left at its default `None`
  (unbounded) - noted in passing during code reading but out of scope for this candidate's specific,
  confirmed finding.
- Real GPU hardware was not available in this environment (per this workspace's `README.md`); all
  reproduction was on SwiftShader (software Vulkan). The bug is a straightforward violation of the
  configured `wgpu::Limits::max_buffer_size`, a spec-level constraint enforced identically by wgpu's
  validation layer regardless of backend/hardware, so this is not expected to be adapter-specific -
  not independently verified on real hardware this session.
