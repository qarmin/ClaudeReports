# VELLO_HYBRID_WGPU_FILTER_TEXTURE_HEIGHT_ASSERT_PANIC

`vello_hybrid`'s wgpu backend has a device-limit/resource-exhaustion gap in its filter-data
texture: `FilterContext::required_filter_data_height` (`sparse_strips/vello_hybrid/src/filter.rs`)
`assert!`s that the required filter-data-texture height fits within
`DeviceLimits::resource_texture_dimension_2d()` (a fixed 4096 cap, see candidate 055) instead of
returning an error. This looked suspicious because candidate 055 already confirmed the identical
pattern for the alpha/encoded-paints/gradient data textures in the sibling file
`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`, and its own "What's not done" section
explicitly flagged the filter-data texture's resize path as a related, not-yet-investigated
`assert!` "worth a maintainer's separate look" - this candidate follows up on exactly that lead,
per this session's task ("vello_hybrid wgpu backend general resource-exhaustion sweep").

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read `sparse_strips/vello_hybrid/src/filter.rs` (`FilterContext`, `GpuFilterData`) against 055's own "What's not done" note | Code reading, ~30 min | Found `required_filter_data_height` has the identical `assert!(height <= resource_texture_dimension_2d, ...)` shape as the three functions 055 fixed - just located in `filter.rs` rather than duplicated inline in `wgpu/mod.rs`'s `maybe_resize_filter_tex` (055's session apparently didn't follow the call into `filter.rs` far enough to spot it, hence its "no corresponding assert at all" note - that note is about the local `maybe_resize_filter_tex` function body, not the callee it invokes). Each filter layer contributes a fixed `GpuFilterData::SIZE_TEXELS = 3` texels (`size_of::<GpuFilterData>() == 48` bytes / 16 bytes-per-texel), added once per filter layer via `Scene::push_filter_layer`/`push_layer(..., filter: Some(_))` - a fully public API. |
| Manual boundary-value arithmetic: with `resource_texture_dimension_2d = 4096` (the fixed cap), the assert fires once `total_texels = filters_len * 3 > 4096 * 4096 = 16,777,216`, i.e. `filters_len >= 5,592,406` | Arithmetic, ~5 min | Computed the minimal filter-layer count needed to cross the bound; confirmed this is a genuinely large but constructible input via the crate's own public `Scene::push_filter_layer`/`fill_rect`/`pop_layer` API (no internal state faked). |
| Manual boundary-value construction + real end-to-end repro: a real `vello_hybrid::Scene` (public API) with 5.7 million `push_filter_layer(Filter::from_primitive(FilterPrimitive::Offset{..}))` + tiny `fill_rect` + `pop_layer` cycles in a single frame, rendered through the real, public `vello_hybrid::Renderer::render` against a real `wgpu::Device`/`Queue` from a real (software) Vulkan adapter (Google SwiftShader, `max_texture_dimension_2d = 8192`) | Release build, real Vulkan adapter via `VK_ICD_FILENAMES` pointed at a SwiftShader ICD | **Reproduced**: panics at `sparse_strips/vello_hybrid/src/filter.rs:670`, `"Filter texture height exceeds resource texture dimensions"`. Reproduces 3/3 runs (deterministic), and again from a completely fresh `git clone` via this candidate's own `justfile`. Boundary checks at 1,000 / 100,000 / 500,000 filters (well under the threshold) all complete without panicking - see `output.txt`. |

Two escalating approaches (code reading following 055's own lead into `filter.rs`, then a real
end-to-end boundary-value repro) were sufficient for a clean, deterministic reproduction on real
(software) GPU hardware - the same tool selection 055 used for the sibling bug, since this is a
plain arithmetic-capacity-vs-hardcoded-constant question, not a memory-safety/UB one that would
need miri/ASAN/fuzzing.

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs, release build, real Vulkan adapter
(SwiftShader) reporting `max_texture_dimension_2d = 8192`. Also reproduces from a completely fresh
`git clone` via this candidate's own `justfile`'s `reproduce-before` recipe.

**Fully reproducible**: **Yes**. This is a single-participant finding (the library's own public
`Renderer::render`/`Scene` API reacting to its own caller's scene data - a scene with many filter
layers - not one participant reacting to a peer's wire input), the same reasoning candidates 026
and 055 already used for this backend. `repro/src/main.rs` drives the actual, real, public
`Scene::push_filter_layer`/`Scene::fill_rect`/`Scene::pop_layer`/`Renderer::render` call sequence a
real caller would make (e.g. an embedder rendering many independently small-blurred/shadowed/offset
elements, such as from a generated SVG or web page with many filtered nodes), through a real
`wgpu::Device`/`Queue` from a real (software) Vulkan adapter - nothing about the filter data, the
scene, or the GPU pipeline execution is faked or hand-constructed to stand in for another
participant.

## Class

crash

## Root cause

`FilterContext::required_filter_data_height` (`sparse_strips/vello_hybrid/src/filter.rs`, called
from `Programs::maybe_resize_filter_tex` in `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`)
computes the filter-data texture's required height from the scene's real filter count
(`total_texels() = filters.len() * GpuFilterData::SIZE_TEXELS`, `SIZE_TEXELS = 3`) and asserts that
height doesn't exceed `DeviceLimits::resource_texture_dimension_2d()` - a value hardcoded to
`min(device_max_texture_dimension_2d, 4096)` by commit f08b480a (see candidate 055's `RESULT.md`
for the full history: that PR's actual fix target was a WebGL2/browser-driver hang tied to texture
*width* near 16k, but the fix was applied to the shared helper used for *both* width and the
height-bound in three other resize functions **and** this one). Every real filter layer a caller
pushes (`Scene::push_filter_layer`, itself calling the public `Scene::push_layer(..., filter:
Some(_))`) that actually contains drawn content triggers exactly one `FilterContext::push` call
during `Renderer::render`'s internal scheduling pass (`ensure_layer_target` in
`sparse_strips/vello_hybrid/src/schedule/mod.rs`) - so a scene with enough distinct filter layers
(the code's own comment calls "more than a few dozen filters in a single scene"
"uncommon", but nothing enforces that assumption) drives `total_texels` past the artificial
4096x4096-texel budget and panics the whole render call, instead of returning
`Result<(), RenderError>` (the exact type `Renderer::render` already returns for every other error
condition) up to the caller.

This is the same root cause commit and the same bug *class* candidate 055 already confirmed for
the alpha/encoded-paints/gradient textures - just a fourth, independent call site of the identical
assert-against-an-artificially-capped-constant pattern that 055's own session didn't reach (055's
own notes explicitly flagged this as unconfirmed and left for a future session).

## Introduced by

f08b480a "vello_hybrid: Limit the texture sizes of data textures to 4096 (#1830)" (2026-08-20) -
confirmed via `git log -S "Filter texture height exceeds resource texture dimensions" --
sparse_strips/vello_hybrid/src/filter.rs`, which returns exactly this one commit. Same introducing
commit as candidate 055 (which fixed the three sibling data textures affected by the identical
change).

## The fix

Mirrors 055's own accepted fix shape exactly, applied to the filter-data texture path that 055 left
untouched: decouple the *height* bound from the artificially-capped
`resource_texture_dimension_2d` (still used, unchanged, for texture *width*) by introducing a
separate `max_resource_texture_height` (the device's real, uncapped
`limits.max_texture_dimension_2d`) and bounding `required_filter_data_height`'s assert against that
instead:

- `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`: `Renderer::new_with` computes
  `max_resource_texture_height` from the real `device.limits()` and threads it through
  `Programs::new` -> `Programs` (new field) -> `resize_data_textures` ->
  `maybe_resize_filter_tex` -> `FilterContext::required_filter_data_height`.
- `sparse_strips/vello_hybrid/src/filter.rs`: `required_filter_data_height` takes the new
  `max_resource_texture_height: u32` parameter and asserts against it instead of
  `resource_texture_dimension_2d`; updated its own doc comment and the existing
  `#[should_panic]` unit test (`filter_data_height_must_fit_resource_texture_limit`) to the new
  two-argument signature and new panic message ("...device texture dimensions" instead of
  "...resource texture dimensions", matching 055's own message-wording convention for the same
  fix shape).
- `sparse_strips/vello_hybrid/src/render/webgl/mod.rs`: its own `maybe_resize_filter_data_tex`/
  `upload_filter_data_texture` call sites were updated to pass `resource_texture_dimension_2d`
  for *both* new-and-old parameters, i.e. **unchanged behavior** for the WebGL2 backend - matching
  055's own explicit scope decision to leave WebGL2 alone, since that's the actual platform the
  underlying driver-hang workaround targets and raising its height bound independently of width
  needs separate judgment this session didn't make either.

Verified against the *original* triggering input (5.7 million filter layers: was a panic at
`filter.rs:670`, now completes and renders without panicking - `output.txt`) **and** a second,
larger input in the same bug class per `CONFIRMATION.md` section 7 (12 million filter layers,
deliberately beyond the *real* device's actual capacity of `4096 (width) * 8192 (height)` texels):
this one still correctly panics after the fix (now at `filter.rs:678`, message updated to "...
device texture dimensions"), proving the fix restores real device capacity rather than deleting
the safety check or silently corrupting the texture size.

- Project test suite, before vs. after (identical in both cases - no regression; the one
  directly-relevant unit test was updated as part of the patch, see above, and still passes):
  - `cargo test --release -p vello_hybrid --lib`: 122 passed; 0 failed -> 122 passed; 0 failed.
  - `cargo check --workspace --all-features`: succeeds, after the patch (0 errors).
  - `cargo build -p vello_hybrid --all-features --target wasm32-unknown-unknown` (the WebGL
    backend's own compile target): succeeds, after the patch (0 errors) - confirms the
    deliberately-unchanged WebGL2 call sites still type-check with the new two-argument
    signature.

Fix scope: only the wgpu (native) backend's actual bound was changed, matching 055's own scope
decision; the WebGL2 backend's behavior is bit-for-bit unchanged (same value passed for both
parameters it now receives).

## What's not done / open questions

- This confirms candidate 055's own open question about `maybe_resize_filter_tex`/
  `required_filter_data_height` lacking the same treatment as the other three resize functions -
  that gap is now closed for the wgpu backend specifically.
- The exact minimal trigger for this bug (~5.59 million distinct filter layers in a single frame)
  is inherently much larger than the sibling bugs 026 (9 images) and 055 (4200 gradients) found in
  this same backend, because each filter layer's contribution to the texel budget
  (`GpuFilterData::SIZE_TEXELS = 3`) is a small fixed constant that can't be amplified by choosing
  different filter parameters - so while this is a real, publicly-reachable panic (nothing in the
  type system or a builder prevents a caller from constructing a scene with millions of filter
  layers, e.g. programmatically generated content with many independently filtered small
  elements), it sits at the far, more-extreme end of "adversarial-but-plausible" compared to its
  siblings. Completing the "after fix, no longer panics" verification for this exact input took
  ~7 minutes of wall-clock time under software (SwiftShader) rendering - see `output.txt` and the
  `justfile`'s own note about this.
- Not evaluated: whether `GaussianBlur`/`DropShadow` filters (as opposed to the cheapest `Offset`
  primitive this candidate's repro uses) reach the same threshold at a meaningfully different
  filter count - `GpuFilterData::SIZE_TEXELS` is a single constant shared by all four filter
  primitive structs (`GpuOffset`/`GpuFlood`/`GpuGaussianBlur`/`GpuDropShadow`, all
  `#[repr(C, align(16))]` and asserted equal-sized via `FILTER_SIZE_BYTES`), so the answer is
  almost certainly "no difference" - not independently re-verified this session.
- Same as 055's own note: the WebGL2 backend's identical assert-against-`resource_texture_dimension_2d`
  pattern (now also present for the filter-data texture, unchanged by this fix) was deliberately
  left as-is, since judging whether raising its height bound independently of width could
  re-trigger the original WebGL2 driver hang is a maintainer decision, not something this session
  investigated.
- Did not sweep the rest of `vello_hybrid`'s wgpu backend beyond this specific lead for further,
  still-unconfirmed resource-exhaustion gaps (the backend's own risk-map note in
  `projects/vello.md` still stands as a good focus area for a future session).
