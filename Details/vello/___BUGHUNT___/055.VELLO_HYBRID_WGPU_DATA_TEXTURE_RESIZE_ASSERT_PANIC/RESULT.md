# VELLO_HYBRID_WGPU_DATA_TEXTURE_RESIZE_ASSERT_PANIC

`sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`'s `maybe_resize_alphas_tex`,
`maybe_resize_encoded_paints_tex`, and `maybe_resize_gradient_tex` each compute a required texture
height from that frame's real data volume (alpha coverage bytes, encoded-paint bytes, gradient LUT
pixels) and, when the current texture is too small, `assert!(required_height <=
resource_texture_dimension_2d, ...)` before growing it - a real (non-`debug_assert!`) assert
reachable from the ordinary public `Renderer::render`/`prepare` path. Commit f08b480a ("Limit the
texture sizes of data textures to 4096 (#1830)") capped `resource_texture_dimension_2d` at a fixed
4096 regardless of the device's actual `max_texture_dimension_2d`, which - since this same value is
reused both as the texture's *width* and as the assert's height bound - shrank the maximum
representable per-frame alpha/paint/gradient data to a fixed ~256-268MB bound on every device, even
ones that natively support far larger textures. This candidate investigates whether that shrunk
bound is reachable through a realistic, non-adversarial use of the public API on real (capable)
hardware, causing a panic that wouldn't have happened before that commit.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Read the three resize functions plus `DeviceLimits::resource_texture_dimension_2d` (`sparse_strips/vello_hybrid/src/render/common.rs`) and its introducing commit (`git log -S "MAX_RESOURCE_TEXTURE_DIMENSION_2D"`) | Code reading + `git show f08b480a`, ~30 min | Confirmed the assert already existed *before* f08b480a, bound against the device's real `max_texture_dimension_2d` (an already-known, but practically almost unreachable, limit given real devices report 8192-16384+). f08b480a's own PR description is explicit that its fix targets a **WebGL2**, browser/driver-specific hang tied to texture *width* near 16k (`gl.texImage2D`/`texSubImage2D` on a ~16365-wide `RGBA32UI` texture freezing Chrome on certain macOS/GPU combinations) - not an overall-data-volume concern ("this issue does not seem to be about the overall amount of data uploaded ... instead really about the maximum width of the texture", quoting the PR). The fix was nonetheless applied uniformly to the shared `DeviceLimits::resource_texture_dimension_2d()` used by *both* the WebGL and the native wgpu backends, capping HEIGHT to the same value as the width-workaround, even for the native wgpu backend where the WebGL2-specific driver bug doesn't apply |
| Read `GradientRampCache`/`gradient_cache.rs` to check whether its own entry-count cap (`retained_count`, derived from `resource_texture_dimension_2d`) already prevents the assert from firing regardless | Code reading, ~20 min | `maintain()` (which evicts down to `retained_count`) only runs once, at the very end of `Renderer::render` (after `programs.prepare()`, which is what calls `maybe_resize_gradient_tex`) - so within a single frame, every gradient cache miss up to that frame's own draw count is added to `luts` *before* any eviction happens. A single `Scene` that draws more distinct max-width gradients in one frame than `retained_count` allows can make `luts_size()` exceed the resource texture's capacity before `maintain()` ever gets a chance to trim it back down |
| Manual boundary-value construction + real end-to-end repro: a real `vello_hybrid::Scene` (public API) filled with many rectangles, each `set_paint`'d with a distinct `peniko::Gradient` (unique color per draw => guaranteed `GradientCacheKey` cache miss; stop spacing chosen via `determine_lut_size`'s own documented formula to force the maximum 4096-wide LUT), rendered through the real, public `vello_hybrid::Renderer::render` against a real `wgpu::Device`/`Queue` obtained from a real Vulkan adapter (Google SwiftShader software implementation, reporting `max_texture_dimension_2d = 8192` - i.e. genuinely double the artificial 4096 cap) | Release build, real Vulkan adapter via `VK_ICD_FILENAMES` pointed at a SwiftShader ICD | **Reproduced**: 4200 such gradients in a single frame panics at `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs:2402`, `"Gradient texture height exceeds resource texture dimensions"`. 4000 gradients (just under the boundary) succeeds. Reproduces 3/3 runs (release build), deterministic (no randomness in the input construction). See `output.txt` |

Two escalating approaches (code reading of the resize/eviction logic, then a real end-to-end
boundary-value repro) were sufficient for a clean, deterministic reproduction on real (software)
GPU hardware - no fuzzing/miri/ASAN needed, since this is a plain arithmetic-capacity-vs-hardcoded-
constant question, not a memory-safety/UB one.

## Status

**Confirmed**. Reproduces deterministically, 3/3 runs, release build, real Vulkan adapter
(SwiftShader) reporting `max_texture_dimension_2d = 8192`. See `output.txt` for the full 3x-before /
3x-after capture plus boundary checks (4000 succeeds both before and after; 8000 panics before the
fix but succeeds after; 8500 - beyond this adapter's real 8192 limit - still correctly panics after
the fix, showing the fix restores real capacity rather than removing the safety check).

**Fully reproducible**: **Yes**. This is a single-participant finding (the library's own public
`Renderer::render`/`Scene` API reacting to its own caller's scene data, not one participant reacting
to a peer's wire input) - `CONFIRMATION.md` section 11's "real second participant" bar doesn't apply
here (same reasoning as candidate 026's own "Fully reproducible" note). `repro/src/main.rs` drives
the actual, real, public `Scene::set_paint`/`Scene::fill_rect`/`Renderer::render` call sequence a
real caller would make, through a real `wgpu::Device`/`Queue` from a real (software) Vulkan adapter
- nothing about the gradient data, the scene, or the GPU pipeline execution is faked or
hand-constructed to stand in for another participant.

## Class

crash

## Root cause

`DeviceLimits::resource_texture_dimension_2d()` (`sparse_strips/vello_hybrid/src/render/common.rs`)
computes `self.max_texture_dimension_2d.min(4096)` and this single value is used for **two**
different purposes in `sparse_strips/vello_hybrid/src/render/wgpu/mod.rs`:

1. The actual *width* of the alpha/encoded-paints/gradient data textures (`create_alphas_texture`,
   etc.) - this is the value f08b480a's PR actually needed to cap, to work around a WebGL2/driver
   hang tied specifically to wide textures.
2. The upper bound each resize function's `assert!` uses for the required **height** (e.g.
   `maybe_resize_gradient_tex`: `assert!(required_gradient_height <= resource_texture_dimension_2d,
   ...)`).

Nothing about the WebGL2 width bug requires bounding a texture's *height* by the same artificially
low number - a rectangular texture's width and height are independent limits on real GPUs, each
bounded separately by the device's own `max_texture_dimension_2d`. By reusing the same capped value
for both, f08b480a shrank the maximum per-frame alpha-byte / encoded-paint-byte / gradient-LUT-pixel
budget from being effectively bounded by the *device's own* (often 8192-16384+) `max_texture_dimension_2d`
down to a fixed ~256-268MB regardless of hardware, on **both** backends - including the native wgpu
backend, which was never affected by the WebGL2-specific driver hang the commit was fixing. A scene
that draws enough distinct large gradients (or, by the same arithmetic, enough alpha-coverage data
or enough distinct paints) in a single frame to exceed that fixed budget now hits a hard `assert!`
panic on hardware that could easily have handled the same scene before this commit.

## Introduced by

f08b480a "vello_hybrid: Limit the texture sizes of data textures to 4096 (#1830)" (2026-08-20) -
this is the commit that shrank the previously device-relative bound (the device's real, uncapped
`max_texture_dimension_2d`, typically 8192-16384+) down to a fixed 4096, making the pre-existing
`assert!`s in the three resize functions reachable at a far smaller, more realistic scene size than
before. The `assert!`s themselves (bounding required height against *some* texture-dimension value)
predate this commit and are not new; what's new is how low that value now is on capable hardware.

## The fix

Added a second field (`max_resource_texture_height`, computed once in `Renderer::new_with` as
`u32::from(device_limits.max_texture_dimension_2d)` - i.e. the real, uncapped device limit) and
threaded it alongside the existing (still-capped) `resource_texture_dimension_2d` into
`maybe_resize_alphas_tex`, `maybe_resize_encoded_paints_tex`, and `maybe_resize_gradient_tex`. Each
function's `assert!` now bounds the required **height** against this real device limit instead of
against the artificially-capped width value; the texture's *width* (and the WebGL2 workaround it
protects) is untouched - still `resource_texture_dimension_2d`, still capped at 4096. Also updated
`max_gradient_cache_size`'s own formula (`resource_texture_dimension_2d * max_resource_texture_height
/ MAX_GRADIENT_LUT_SIZE`) so the gradient cache's own entry-count budget grows to match the larger
available height, rather than staying artificially small.

Verified: re-ran the same repro (`NUM_GRADIENTS=4200`) against the patched tree - no longer panics,
3/3 runs (see `output.txt`, section 2). Per `CONFIRMATION.md` section 7, also verified the fix isn't
just "raise the threshold arbitrarily": `NUM_GRADIENTS=8000` (double the old artificial cap, still
within this adapter's real 8192 limit) now succeeds, while `NUM_GRADIENTS=8500` (genuinely beyond
this adapter's real limit) still correctly panics after the fix, with the assert message now
correctly saying "device texture dimensions" rather than "resource texture dimensions" - the fix
restores real device capacity rather than removing the safety check or silently corrupting the
texture size.

Fix scope: only the wgpu (native) backend's copy of these three functions was changed. The
webgl/mod.rs backend has the identical assert-and-cap pattern (see "What's not done" below) and was
deliberately left untouched, since it's the platform f08b480a's own workaround actually targets -
raising its height bound independently of width needs separate judgment about whether the WebGL2
driver hang could plausibly be re-triggered by a taller-but-not-wider texture, which this session
did not investigate.

- Project test suite: `cargo test --release -p vello_hybrid --lib` - **122 passed, 0 failed** both
  before and after the patch (no regressions).

## What's not done / open questions

- `sparse_strips/vello_hybrid/src/render/webgl/mod.rs` has the identical
  `maybe_resize_alphas_tex`/`maybe_resize_encoded_paints_tex`/`maybe_resize_gradient_tex`
  assert-against-`resource_texture_dimension_2d` pattern, sharing the same
  `DeviceLimits::resource_texture_dimension_2d()` helper. This candidate did not attempt a fix
  there, since that's the actual platform (WebGL2/browser) the underlying driver bug was reported
  on - whether it's safe to let its data textures grow taller than 4096 without also risking the
  original browser hang is a judgment call this session left to the project maintainers rather than
  guessing at.
- The `maybe_resize_filter_tex` function in `wgpu/mod.rs` computes a required filter-texture height
  the same way but has no corresponding `assert!` at all (it just always resizes) - not part of this
  candidate's confirmed finding, but worth a maintainer's separate look at whether that's
  intentional (unbounded growth) or an oversight in the other direction.
- Not evaluated: whether a similarly-sized alpha-coverage or encoded-paint scene (as opposed to the
  gradient-cache path this candidate's repro exercises) is equally easy to construct in a
  "realistic" scene, versus needing an even more contrived one - the arithmetic bound is identical
  for all three (`~256-268MB` on a 4096-capped device) but this session only built a working repro
  for the gradient case.
