# GPU_BUFFER_BITPACK_STRUCT_SIZE_ASSUMPTION

`vello_encoding/src/encoding.rs`'s `Encoding::encode_image` packs several fields into two `u32`
words with no static or runtime check that each field's assumed bit width still holds - flagged
in linebender/vello#1259 (raised from PR #1173's review): "(Un)packing code feels unsafe, as there
are some assumptions of types that can change. We should at least have some const asserts here for
size assumptions." The task was to check whether any *currently defined* type already violates
this assumption before treating it as purely future-proofing.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Manual bit-budget audit of `sample_alpha`'s four packed enum fields (`format`, `alpha_type`, `quality`, `x_extend`/`y_extend`) against each `peniko` enum's current variant count | Read `peniko-0.6.1` source for `ImageFormat`/`ImageAlphaType`/`ImageQuality`/`Extend` discriminants | No current violation: `ImageAlphaType` (2 variants) exactly fits its 1 allocated bit, `ImageQuality`/`Extend` (3 variants each) exactly fit their 2 allocated bits, `ImageFormat` (2 variants) has 17 bits allocated (hugely oversized). All four are correct today, but three of the four have zero spare capacity for even one more variant. |
| Same audit extended to the sibling `width_height` field (packs `image.width`/`image.height`, plain `u32`s, not enum discriminants) in the same function | Read the encode side (`(brush.image.width << 16) \| (brush.image.height & 0xFFFF)`) and the shader's unpack side (`fine.wgsl`'s `read_image`) | Found a live violation: `width` is never masked/validated before the `<< 16` shift, so any `width` (or, via the `& 0xFFFF` mask, `height`) >= 65536 silently corrupts the packed word via ordinary `u32` wraparound - not a hypothetical future-type-size issue, a defect in the current code given legitimate `u32` inputs it already accepts. |
| Standalone `repro/` driving the real public `vello::Scene::draw_image` -> `Encoding::encode_image` with a real `peniko::ImageData` at boundary widths/heights (1, 65535, 65536, 65537, 131072, 100000x50000, height-overflow) | debug build, 5 repeat runs; release build, 1 run | Deterministic, 5/5 identical: corruption for every width or height >= 65536, e.g. width=131072 and width=65536 both decode back to width=0 (aliasing two very different declared sizes to the same wrong value). See `output.txt`. |
| Extended the same repro to run the REAL end-to-end `vello_encoding::Resolver::resolve` pipeline (the exact call `vello::Renderer` makes before ever touching the GPU) and inspect the actual final packed GPU buffer bytes, not just the intermediate `Encoding.draw_data` | Same boundary inputs, real `Resolver::resolve` call, no faked state | The corruption never reaches the final GPU-submitted buffer through `vello`'s own real pipeline: `vello_encoding::image_cache::MAX_ATLAS_SIZE` is a hardcoded `8192`, and `Resolver::resolve` explicitly zeroes both the atlas-xy and `width_height` words whenever an image fails to fit the atlas - which is guaranteed for any dimension >= 65536 (the threshold that triggers this packing bug), since 65536 > 8192 unconditionally. See "Scope of the fix" below. |

## Status

**Confirmed** (as a real, deterministic defect in `vello_encoding::Encoding::encode_image`'s
public API, using only the real function and legitimate `u32` inputs - no hand-constructed
internal state). Reproduces 5/5 debug-build runs and 1/1 release-build run, byte-for-byte
identical each time; no seed/layout dependence (it's pure integer arithmetic on the exact input
given).

**Fully reproducible: Yes.** `repro/` calls only the real, unmodified public API
(`vello::Scene::draw_image`, backed by `vello_encoding::Encoding::encode_image`, and separately
`vello_encoding::Resolver::resolve`) with a real `peniko::ImageData` whose `width`/`height` are
exactly the kind of `u32` value a real caller could legitimately construct (these fields are
public, untyped `u32`, with no documented upper bound anywhere in `peniko` or `vello_encoding`).
No second participant is impersonated anywhere in this repro - see "Scope of the fix" below for
why this is nonetheless not the same claim as "this is externally visible through `vello`'s
shipped renderer today."

## Class

logic / data corruption in a bit-packing helper (not a memory-safety issue - `width_height`/
`sample_alpha` are plain `u32` fields, not raw pointers or slice indices).

## Root cause

`Encoding::encode_image` packs `DrawImage::width_height` as `(image.width << 16) |
(image.height & 0xFFFF)`, assuming both `image.width` and `image.height` fit in 16 bits each
(the shader's `read_image` in `fine.wgsl` unpacks them the same way: `width_height >> 16u` /
`width_height & 0xffffu`). Neither side ever validates this assumption. `peniko::ImageData`'s
`width`/`height` are plain public `u32` fields with no upper-bound documented or enforced
anywhere upstream, so a caller can legitimately construct an `ImageData` with, say,
`width = 100_000`, and `encode_image` will silently produce a corrupted `width_height` word via
ordinary unsigned-integer overflow (a left shift that pushes bits past bit 31 off the top, and a
mask that drops height's own high bits) rather than rejecting or clamping the value. Separately,
the four enum-packed fields in the same function's `sample_alpha` word (`format`/`alpha_type`/
`quality`/`x_extend`/`y_extend`) currently all fit their allocated bit widths exactly (see "What
was tried" above), but three of the four (`alpha_type`, `quality`, `x_extend`/`y_extend`) have no
spare capacity at all - a single new variant added to `peniko::ImageAlphaType`, `ImageQuality`, or
`Extend` in a future dependency bump, with no corresponding change to this function, would
silently alias into the low bits of the *next* field over rather than failing to compile or even
panicking at runtime.

### Scope of the fix (why this doesn't currently misrender through `vello::Renderer`)

The full real pipeline (`vello::Renderer::render_to_texture`/`render_to_surface`, which always
calls `vello_encoding::Resolver::resolve` before submitting anything to the GPU) does not
currently expose this specific corruption as an externally visible rendering defect. This is
incidental, not a deliberate guard against this bug: `vello_encoding::image_cache::MAX_ATLAS_SIZE`
is a hardcoded `8192`, and `resolve_pending_images`/`resolve_patches` explicitly overwrite *both*
the atlas-xy and `width_height` words with zero for any image that fails to fit the atlas - which
is unconditionally true for any image with either dimension >= 65536, since 65536 is always
greater than the atlas's own 8192 cap. Verified directly: running the real `Resolver::resolve`
end-to-end on every boundary case in `repro/` shows the final GPU-submitted buffer is
`0x00000000` for every input with a dimension > 8192, corrupted or not (compare `width=65535`,
which round-trips correctly through `encode_image` but is *still* zeroed by `resolve()`, to
`width=65536`, which is corrupted by `encode_image` and *also* zeroed by `resolve()` - the two
produce an identical final result). So through `vello`'s own shipped renderer, this defect
currently produces no different externally-visible outcome than any other oversized image already
produces via the unrelated atlas-capacity check.

This does **not** make the underlying defect harmless in general: `vello_encoding` is itself a
separately published, general-purpose crate ("Vello types that represent the data that needs to be
rendered") with a public `Encoding`/`encode_image` API; any consumer building their own renderer
directly on `vello_encoding::Encoding.draw_data` without going through `Resolver::resolve` and its
atlas-capacity check would see the corrupted value directly. The fix (below) hardens
`encode_image` itself rather than relying on the incidental, unrelated atlas-size mitigation.

## Introduced by

165b3a08 "Let's add images" (2023-03-09) - the `(image.width << 16) | (image.height & 0xFFFF)`
packing pattern (unmasked `width`, masked `height`, no bounds validation on either) has existed
unchanged in every subsequent refactor of this function since image support was first added to
the "classic" vello renderer, confirmed via `git log -S "image.width << 16" --reverse`.

## The fix

Two independent, additive changes to `vello_encoding/src/encoding.rs`'s `encode_image`, both
scoped to this one function:

1. Module-scope `const _: () = { assert!(...) }` block verifying each of the four enum-packed
   `sample_alpha` fields' currently-highest variant still fits its allocated bit width (addresses
   the "const assertions" request in linebender/vello#1259 directly - these fire on every build,
   including a plain `cargo build`, not just `cargo test`, unlike the two pre-existing
   exhaustive-match tests `ensure_image_quality_values`/`ensure_extend_values` in this same file's
   test module, which only run under `cargo test` and don't cover `ImageFormat`/`ImageAlphaType`
   at all).
2. A `debug_assert!` plus a defensive `.min(0xFFFF)` clamp on `width`/`height` before packing
   `width_height`, so a debug build fails loudly with a clear message instead of silently
   producing a corrupted value, and a release build clamps deterministically to the maximum
   representable dimension instead of wrapping to an arbitrary, potentially aliasing value.

Verified against more than the one initially-tried boundary value (per CONFIRMATION.md section 7):
`repro/` exercises width-only overflow (65536, 65537, 131072), a combined large width+height
(100000x50000), and height-only overflow (1x65536) - all five out-of-range cases are caught
identically by the `debug_assert!` in the debug build, and all five clamp identically (bounded,
non-aliasing) in the release build. See `output.txt` for the full before/after transcript.

- Project test suite: **95 passed, 1 ignored, 0 failed, both before and after** (identical test
  names/counts) - `cargo test -p vello_encoding -p vello -p vello_tests --tests`, with
  `VELLO_SKIP_LFS_SNAPSHOTS=all` set (this checkout has no `git-lfs` binary, so the `emoji.rs`
  snapshot-comparison tests are steered to skip real-image comparison rather than failing on
  unreadable LFS pointer files - unrelated to this candidate). Breakdown: `vello_encoding` unit
  tests (31), `vello` (0 unit tests), `vello_tests` integration tests across
  `compare_gpu_cpu`/`emoji`/`hinting`/`known_issues`/`property`/`regression`/`smoke_snapshots`/
  `snapshot_test_scenes` (9+1 ignored, 6, 4, 2, 6, 10, 8, 19).

## What's not done / open questions

- The `width_height`-overflow fix only covers the one function found to actually violate the
  assumption today (`encode_image`'s image-dimension packing). The sibling `sparse_strips/
  vello_hybrid` renderer has its own, separately-implemented `pack_image_size`/
  `pack_image_offset` helpers (`sparse_strips/vello_hybrid/src/render/common.rs`) that take
  `width`/`height` as `u16` at the type level (not `u32`), so the same class of bug is not
  reachable there by construction - not re-verified further since the type signature itself rules
  it out, unlike the `u32`-typed `vello_encoding` path.
- Whether to also change `MAX_ATLAS_SIZE`'s relationship to the 16-bit packing limit (e.g. a
  `const_assert!` tying `MAX_ATLAS_SIZE <= 0xFFFF` so a future maintainer raising the atlas cap
  can't silently reintroduce a *reachable* version of this same corruption) was not attempted this
  session - flagged as a related follow-up, not fixed here, since it's a different function
  (`vello_encoding::image_cache`) than the one this candidate investigated.
- No fuzzing was run against `encode_image`; the boundary-value approach (per HEURISTICS.md's
  guidance for "arithmetic on untrusted sizes/casts between integer widths") was sufficient to
  reproduce deterministically without it.
