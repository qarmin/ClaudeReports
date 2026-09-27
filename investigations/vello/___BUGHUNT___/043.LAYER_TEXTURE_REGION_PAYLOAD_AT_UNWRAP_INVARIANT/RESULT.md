# LAYER_TEXTURE_REGION_PAYLOAD_AT_UNWRAP_INVARIANT

`sparse_strips/vello_hybrid/src/draw.rs:436-441`, `LayerTextureRegion::payload_at` computes
`u16::try_from(x as i32 + shift.0).unwrap()` (and the same for `y`), where
`shift = self.geometry_shift()` (`target.rs:298-311`) is
`texture.rect.x0 as i32 - layer_bbox.x0 as i32`, which can be negative when the layer's bbox
doesn't start at the render target's origin. The comment above it says this "should never fail"
because callers only sample coordinates within the layer bbox, but there is no
`debug_assert`/checked arithmetic enforcing that invariant at the call boundary - it is a
cross-module contract spanning `draw.rs` (the two call sites), `target.rs`
(`LayerTextureRegion`/`geometry_shift`), and `schedule/mod.rs` (which constructs/threads
`LayerTextureRegion` values through scheduling). The candidate flagged this as fragile to future
refactors and asked for stress-testing with layers whose bbox touches render-target edges or is
asymmetrically clipped.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Static trace of every call site of `LayerTextureRegion::payload_at` (`draw.rs:323` in `push_layer_fill`'s rect-blit branch, `draw.rs:342` in `push_layer_fill_segment`) and of the `x`/`y` values' provenance back to `RectU16::intersect` (`vello_common/src/geometry.rs:208-215`, `x0 = max(self.x0, other.x0)`) | Manual code reading | Both call sites pass coordinates derived from `sample_bbox = sample.layer_bbox.intersect(self.state.target_bbox)` (or a `visit_strip_fill_segments` tile range clipped to `sample_bbox.to_tile_bounds()`), and `intersect`'s `x0`/`y0` are computed via `max(...)`, so `sample_bbox.x0 >= layer_bbox.x0` and `sample_bbox.y0 >= layer_bbox.y0` hold by construction for every current caller - `crop_to` (`target.rs:257-267`) also preserves `geometry_shift` exactly (`texture.rect.x0` and `layer_bbox.x0` are shifted by the same clip delta), so re-cropping a region doesn't change the invariant either. This makes the invariant look structurally guaranteed today, not just "true in practice" - matching the candidate's own note that it traced current call sites and found the invariant holds. |
| Randomized fuzz-like harness (own PRNG, no external crate) added temporarily to `vello_hybrid`'s internal test suite (`src/schedule/schedule_tests.rs`), driving the real public `Scene` API (`push_layer`/`fill_rect`/`push_opacity_layer`) through the crate's internal `Schedule::try_new` (the same function the real `wgpu`/WebGL `Renderer::render` calls) - randomized canvas size (1-96px), draw/clip rects spanning -20..canvas+20 (so clips routinely start off-canvas or exceed it), up to depth-7 nested layers, blend mode in {none, `Mix::Multiply`/`SrcOver`, isolated `Compose::Clear`}, opacity in {0.0, 1.0, random, none}, `texture_size` in 4-64px, `max_textures` in 1-8, `use_depth_buffer` true/false | Debug build (`cargo test`, `debug-assertions`/`overflow-checks` on by default), 500,000 iterations, ~22s, fixed seed `0x1234_5678_9ABC_DEF0` | 0 panics. (An earlier 20,000/200,000-iteration run with a different seed and a variant that also probed canvas sizes near `u16::MAX` did hit a **different, already-acknowledged** panic in `vello_common::util::RectExt::snap_to_tile_coordinates` - own comment: "This method will panic if we have a viewport of size u16::MAX and draw at the very edge, but better than returning a wrong result" - unrelated to this candidate's `payload_at` suspect; excluded from later runs, see "New suspects" below.) |
| Manual boundary-value construction: 3 targeted deterministic scenes added as `#[test]`s in the same file - (1) a clip rect exactly flush with the bottom-right corner of a 16x16 canvas with an 8px texture atlas page, (2) an asymmetric clip nested inside an isolated (`Compose::Clear`) blend layer with a further nested child clip at the far edge, (3) a layer clipped to a single 4x4-pixel tile at the far corner of a 64x64 canvas with a tiny 4px texture page (maximizes the magnitude of the negative `geometry_shift` relative to the sampled coordinates) | Debug build, `cargo test` | 0 panics across all 3 scenarios and all `RootTarget` variants tried (`UserSurface`, `AtlasLayer`) and both `use_depth_buffer` settings. |
| Positive control: called `LayerTextureRegion::payload_at` directly (it's only `fn`-private, reachable from `draw.rs`'s own `#[cfg(test)]` module) with `layer_bbox = (20,20)-(30,30)` and `x = 10` (deliberately less than `layer_bbox.x0`, i.e. deliberately violating the documented invariant) | Debug build, `cargo test`, `#[should_panic(expected = "TryFromIntError")]` | Panics exactly as expected: `called Result::unwrap() on an Err value: TryFromIntError(NegOverflow)` at `draw.rs:440`. This confirms the panic mechanism is real and that the fuzz/manual tests above were exercising a check that actually bites when the invariant is broken, not silently passing for an unrelated reason. |

## Status

**Not reproducible.** Covered: 500,000 randomized scene schedules (debug-assertions build, fixed
seed, deterministic, single run - not re-run a second time since the earlier same-shaped 200,000
and 20,000-iteration runs under different seeds/parameter ranges already agreed at 0 panics) plus
3 hand-constructed boundary scenes targeting exactly the "layer bbox touches the render-target
edge" / "asymmetrically clipped" cases the candidate asked about, plus a positive control proving
the suspect code does panic when its invariant is actually violated. No input constructed through
any real scene-construction call path (the only way a real caller ever produces a
`LayerTextureRegion`) violated the invariant.

**Fully reproducible**: N/A (no bug found - negative result). All scene-construction and
scheduling exercised in this session went through `vello_hybrid`'s real internal `Schedule`
building code (`Schedule::try_new`, the same function `Renderer::render`/`render_to_atlas` call)
driven by the crate's real, public `Scene` API (`push_layer`, `fill_rect`, `push_opacity_layer`) -
this is a single-process, single-participant question (does the scheduler's own geometry math
ever violate its own invariant), not a finding about one participant reacting to another's input,
so `CONFIRMATION.md` section 11's "real second participant" bar does not apply. The one exception
is the positive-control test, which intentionally calls the private `payload_at` method directly
with a hand-constructed, invariant-violating `LayerTextureRegion` - that test exists only to prove
the panic mechanism itself is live, not to claim a real bug; it is explicitly not offered as
evidence of reachability.

## Class

N/A (not Confirmed - see Status).

## Root cause

Not a reachable defect as of this checkout: every current call site of `LayerTextureRegion::payload_at`
passes an `(x, y)` pair derived from intersecting the region's `layer_bbox` with some other bound
(`target_bbox`, or a `visit_strip_fill_segments` tile range clipped to that intersection), and
`RectU16::intersect`'s `x0`/`y0` are `max(self.x0, other.x0)` (never `min`), so the result can
never have a smaller `x0`/`y0` than the original `layer_bbox`. Since `geometry_shift() =
texture.rect.x0 - layer_bbox.x0` is computed from the *same* (uncropped) `layer_bbox` that bounds
every sampled coordinate from below, `x + shift.0 = texture.rect.x0 + (x - layer_bbox.x0) >=
texture.rect.x0 >= 0` always holds, and is bounded above by `texture.rect.x1 <= u16::MAX` for the
same reason (the sampled coordinate never exceeds `layer_bbox`'s extent either). `LayerTextureRegion::crop_to`
(used when composing a blend's clip strips) recomputes both `texture.rect.x0` and `layer_bbox.x0`
by the same clip delta, so `geometry_shift` is unchanged by cropping and the same argument still
applies afterward. In short: the invariant the comment claims is not merely "true in practice" -
it's implied by `intersect`'s `max`-based definition plus every caller routing its coordinates
through an intersection with `layer_bbox` before calling `payload_at`. The candidate's own
"traced current call sites and the invariant does appear to hold" assessment is confirmed; nothing
found this session weakens it further (no code path bypasses the intersection).

## Introduced by

N/A - not applicable, since Status is Not reproducible (no bug found to trace to an introducing
commit). For reference, `LayerTextureRegion`/`payload_at`/`geometry_shift` were all added in
c3343862 "vello_hybrid: A complete rewrite of the core logic (#1759)" (2026-07-24), the only hit
for `git log -S payload_at -- sparse_strips/vello_hybrid/src/draw.rs`; not investigated further
since there is nothing to root-cause.

## The fix

No fix attempted - there is nothing to fix (see Status/Root cause): the code behaves safely for
every real trigger path found. A defensive hardening a maintainer might still want (not attempted
here, since it's a robustness suggestion rather than a fix for an observed defect): replace the
two `u16::try_from(...).unwrap()` calls with `x.checked_add_signed(...)`-style checked arithmetic
plus a `debug_assert!`, or add a one-line `debug_assert!(self.layer_bbox.contains(x, y))` at the
top of `payload_at` - this would turn a hypothetical future violation (e.g. a refactor that skips
the `intersect` step) into an immediate, precisely-located debug-build panic instead of relying on
readers noticing the cross-module contract, without changing today's behavior at all (the
invariant already holds for every real caller, so the assertion would never fire currently). No
source files in the main project were modified for real; all exploration used a disposable git
worktree (`/tmp/vello-worktrees/wt043`, cleaned via `git checkout -- .` after each experiment) with
temporary `#[test]`s that were never committed or left behind (see `instrumentation.diff` in this
folder for exactly what was added and removed).

- Project test suite: N/A - no fix was made, so no before/after run was needed for this candidate.
  (The temporary instrumentation itself, while present, ran clean: 5 passed, 0 failed within the
  affected test binary - see `output.txt`.)

## What's not done / open questions

- Did not build a standalone external `repro/` crate: `Schedule`, `ScheduleStorage`, `RootTarget`,
  and `PaintResolver` are all crate-private in `vello_hybrid` (not re-exported from `lib.rs`), so
  the only real entry points reachable from outside the crate are `Renderer::render`/`render_to_atlas`
  (wgpu) or the WebGL equivalent, both of which need a real `wgpu::Device`/GL context. Since this
  is a negative result and the suspect code (`Schedule::try_new`'s CPU-side strip building) runs
  entirely before any GPU submission, the crate's own internal test harness (`test_support::SceneCase`,
  already used by `vello_hybrid`'s own `schedule_tests.rs`) exercises the identical code path with
  less setup risk than standing up a software `wgpu` device; a software-Vulkan (SwiftShader) ICD is
  available in this environment (`/opt/ms-playwright/chromium-1234/chrome-linux64/libvk_swiftshader.so`,
  used successfully by an earlier candidate, 026) if a future session wants a fully-external repro
  regardless.
- Did not check the `BlendOp`/`crop_to` composition path's own use of `geometry_shift()`
  (`schedule/mod.rs` around line 616-680, used for blend-clip strips) as thoroughly as
  `push_layer_fill` - that path uses `RectU16::shift` (which *clamps* rather than panics) for
  geometry, not `payload_at`, so it isn't this candidate's suspect code, but wasn't separately
  fuzzed; the depth-7 nested-layer fuzz above does exercise it incidentally (isolated
  `Compose::Clear` layers trigger it) without finding a problem.
- New suspect noticed while fuzzing (not investigated this session, not part of this candidate):
  `vello_common::util::RectExt::snap_to_tile_coordinates` (`vello_common/src/util.rs:178-190`) has
  its own acknowledged, un-guarded panic ("This method will panic if we have a viewport of size
  u16::MAX and draw at the very edge") reachable by constructing a canvas near `u16::MAX` with a
  draw near its edge - hit incidentally by an earlier, wider-ranging draft of this session's fuzz
  harness before that canvas-size range was removed to keep this candidate focused. Left as an
  open lead for a future candidate.
