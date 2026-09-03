# VELLO_HYBRID_ALLOCATE_REGIONPROPS_ALLOCATION_SIZE_EXPECT_PANIC

`sparse_strips/vello_hybrid/src/schedule/allocate.rs:150-157`, `RegionProps::allocation_size()`,
converts a detected u16-domain overflow (`size + 2*padding > u16::MAX`) into
`.expect("layer allocation size exceeds the u16 atlas domain")` instead of returning
`IntermediateTextureError::TooLarge`, which the same crate does surface a few lines away in
`Cursor::allocate_layer` for a plain (non-overflowing) too-large allocation. `size` comes from a
layer's caller-influenced `RectU16` bounding box and `padding` is `FILTER_ATLAS_PADDING` (= 6, i.e.
`MAX_KERNEL_SIZE / 2` with `MAX_KERNEL_SIZE = 13`) for any filter layer (blur, etc.). Flagged as the
same bug class as confirmed candidate 052 (`FILTER_DATA_SNAPPED_PADDING_U16_SATURATION`) but at a
different call site.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual boundary-value / call-graph analysis | Traced every caller of `RegionProps::allocation_size()` back to the public API, by hand, across `allocate.rs`, `cursor.rs`, `schedule/mod.rs`, `render/common.rs`, `render/wgpu/mod.rs`, `render/webgl/mod.rs` | Found that every `bbox` that can reach `RegionProps::allocation_size()` is first validated, with the exact same size+padding arithmetic (in safe `u32` space), by `LayersConfig::required_intermediate_texture_size()` in `render/common.rs:140-190`, which is called with `?` immediately before `Schedule::try_new()` (`render/wgpu/mod.rs:467-469` / equivalent webgl path) — the sole call site of the scheduler. If that check would fail (including the exact overflow condition this candidate describes), `render_scene` returns `Err(IntermediateTextureError::TooLarge)` and the scheduler — hence `RegionProps::allocation_size()` — is never invoked. See "Root cause" below for the full chain. |
| Debug-assertions unit tests, run via `cargo test -p vello_hybrid --lib` | debug build, `cargo test` | (a) Added a temporary `#[should_panic]` test directly constructing `RegionProps { size: SizeU16::from_wh(u16::MAX - 5, 10), padding: FILTER_ATLAS_PADDING }` and calling `allocation_size()` — confirmed the `.expect()` panic fires exactly as described when this private struct is reached directly with an overflowing size (mechanism is real). Reverted after the run, not shipped. (b) Ran the project's own pre-existing test `render::common::tests::required_intermediate_texture_size_rejects_filter_padding_overflow` (uses `largest_filter_layer_size = SizeU16::from_wh(u16::MAX, 10)`, the same overflow-inducing size) — passes, confirming the guard rejects this exact case with `IntermediateTextureError::TooLarge` rather than panicking. Both (a) and (b) passed as expected, corroborating the call-graph analysis. |
| Real end-to-end GPU repro attempt (per CONFIRMATION.md section 11) | Looked for a real `wgpu`/`Renderer::new` device in this sandbox | No GPU adapter available at all (`/dev/dri` absent, no `vulkaninfo`/`glxinfo`), so an actual `Renderer::new(&device, ...)` + `render()` call could not be executed here. Moot for this candidate regardless — see Root cause: the guard's correctness does not depend on which device is used, only on `device.limits().max_texture_dimension_2d`, and `Renderer::new`/`new_with` (the only real constructors, for both the wgpu and webgl backends) unconditionally derive `DeviceLimits` from that real, adapter-reported value and unconditionally call `MemorySettings::normalize()` before ever touching a `Scene`. There is no public constructor that skips this. |

## Status

**Not reproducible.**

The `.expect()` panic mechanism itself is real (confirmed by direct unit invocation of the private
`RegionProps::allocation_size()`), but it is structurally unreachable from any caller going through
the crate's real, public API. See Root cause for the exact chain that prevents it.

**Fully reproducible**: N/A (not Confirmed).

## Class

N/A (not Confirmed).

## Root cause (why this turned out to be safe)

Every layer `bbox` that ends up in a `RegionProps` passed to `Atlases::allocate_layer` /
`RegionProps::allocation_size()` is `recorded_layer.bbox`, set once in
`vello_common::record::CommandRecorder::pop_layer` (`sparse_strips/vello_common/src/record.rs:310-353`)
— for a filter layer this is `placement.pixmap_bbox`, and the very same value is folded into
`recorder.largest_filter_layer_size` (`record.rs:342-350`) via `.max(...)` over every filter layer in
the scene. `LayerAllocationRequest::new` (`allocate.rs:111-134`) later derives `RegionProps.size`
straight from that same `bbox`, and `RegionProps.padding` is `FILTER_ATLAS_PADDING` for exactly the
same `RecordedLayerKind::Filter` case that fed `largest_filter_layer_size`.

Before the scheduler that eventually calls `RegionProps::allocation_size()` is ever built,
`render_scene` (`render/wgpu/mod.rs:452-490`, and the equivalent webgl path) calls:

```
let required_texture_size = self.layers_config.required_intermediate_texture_size(&scene.recorder)?;
...
let schedule = Schedule::try_new(..., texture_size, ...)?;
```

`LayersConfig::required_intermediate_texture_size` (`render/common.rs:140-190`) computes, for the
scene-wide *largest* filter/layer size, `checked_size(u32::from(size.width()) + filter_padding, ...)`
— i.e. the exact same `size + 2*padding` sum `RegionProps::allocation_size()` computes, but in `u32`
space (so it can never itself overflow) — and returns `Err(IntermediateTextureError::TooLarge)` if
that sum exceeds `LayersConfig::max_texture_size`. Because the `bbox` behind every individual filter
layer's `RegionProps` is always `<=` the scene-wide `largest_filter_layer_size` that this check
validates, if the check passes then no individual layer's `size + 2*padding` can exceed
`max_texture_size` either — and `max_texture_size` is itself always `<= u16::MAX`, so
`RegionProps::allocation_size()` can never actually overflow for any bbox that survives this guard.
The `?` on `required_intermediate_texture_size(...)` means that if the check *would* fail (including
this exact overflow condition), `Schedule::try_new` — and therefore the panic site — is never
reached at all.

Crucially, `max_texture_size` cannot be inflated by a caller to defeat this: `MemorySettings::normalize`
(`render/common.rs:112-136`) unconditionally clamps it to `device_limits.max_texture_dimension_2d`,
and both real constructors — `Renderer::new`/`new_with` in `render/wgpu/mod.rs:171-199` and the
equivalent in `render/webgl/mod.rs` — unconditionally call `normalize()` with `DeviceLimits` built
directly from the real device/adapter's own reported `max_texture_dimension_2d` (via
`u16::try_from(limits.max_texture_dimension_2d).unwrap_or(u16::MAX)`). There is no public
constructor that skips this normalization step. Even in the degenerate case where a hypothetical
adapter reports a limit `>= u16::MAX` (triggering the `unwrap_or(u16::MAX)` fallback), the guard
above still holds: it operates in `u32` space and only lets through sums that are already `<=
max_texture_size <= u16::MAX`, which by definition cannot overflow `u16`.

The panic-capable `.expect()` in `RegionProps::allocation_size()` and the protecting guard in
`required_intermediate_texture_size` were introduced in the **same commit**, 80062c1a "vello_hybrid:
Use `u16` consistently throughout the codebase (#1826)" (`git log -S` on both the panic message and
on `required_intermediate_texture_size` both point at 80062c1a) — i.e. this looks like a deliberate
"validate at the public-facing boundary, `.expect()` as a should-never-happen internal invariant
once inside the scheduler" design, and the boundary validation is airtight for every path this
session could find. This is a materially different situation from confirmed candidate 052
(`FILTER_DATA_SNAPPED_PADDING_U16_SATURATION`), where the equivalent finding was a genuine,
un-guarded silent-wrong-value bug rather than a panic shadowed by an earlier, correct check.

## Introduced by

80062c1a "vello_hybrid: Use `u16` consistently throughout the codebase (#1826)" — introduced both
the `.expect()` in `RegionProps::allocation_size()` and the protecting
`required_intermediate_texture_size` guard in the same change (not that it matters much here since
the finding did not reproduce, but recorded per the candidate-loop's best-effort step).

## The fix

No fix attempted — nothing to fix. The `.expect()` is unreachable dead code from any caller
respecting the crate's real public API (`Renderer::new`/`new_with`), so there is no bounded change
needed to prevent an externally-triggerable panic here. (A cosmetic improvement — replacing the
`.expect()` with a `debug_assert!`/comment stating the invariant it now relies on — could be
considered by maintainers for readability, but that is a style opinion, not a bug fix, and was not
applied.)

- Project test suite: N/A — no fix was made, so no before/after comparison applies. For reference,
  `cargo test -p vello_hybrid --lib schedule::allocate::tests` and
  `cargo test -p vello_hybrid --lib render::common::tests` both pass on the unmodified tree (6/6 and
  8/8 tests respectively, including the pre-existing
  `required_intermediate_texture_size_rejects_filter_padding_overflow` test that already exercises
  this exact scenario).

## What's not done / open questions

- Did not attempt a genuine end-to-end GPU repro (real `wgpu::Device` + `Renderer::render()`) since
  no GPU/adapter is available in this environment; per the root-cause analysis above this would not
  have changed the outcome, since the guard's correctness is a pure arithmetic property independent
  of which real device/adapter is used.
- Did not check the webgl backend's `required_intermediate_texture_size` call site line-by-line to
  the same depth as the wgpu one, but it shares the identical `LayersConfig`/`MemorySettings`/
  `normalize()` code in `render/common.rs`, and its `DeviceLimits` construction
  (`render/webgl/mod.rs:364-369`) uses the identical `u16::try_from(...).unwrap_or(u16::MAX)`
  pattern, so the same argument applies.
- Did not exhaustively search for every other call site that could reach `Cursor::allocate_layer`
  bypassing `required_intermediate_texture_size` (there is only one call site of `Schedule::try_new`
  in the crate, found via `grep`, so this is believed to be complete, but a future session could
  double-check if the scheduler internals change).
