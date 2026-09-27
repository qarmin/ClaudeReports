# 86 - apply candidate 054's `visit_opacity` early-out for real, on a scene that forces Skia's layer path (candidate 068 follow-up)

Candidate 054 wrote (but never benched on Skia, and never applied to real source) an early-out
in all three renderers' `visit_opacity`: return immediately when `opacity <= 0.0`, before
`apply_opacity`/`need_layer`/any layer setup. Candidate 068 confirmed by code reading plus a
temporary debug counter that Skia's own per-draw-method alpha gate (its own, separate fix) is a
no-op whenever an `Opacity`-wrapped subtree has more than one item total (a drop-shadow sibling, a
`visible:`-injected `Clip`, or genuinely more than one child) - `perf_test`'s only existing
opacity-capable scenes happen to always hit that multi-item case, so `Opacity::need_layer` always
returns `true` there and Skia's `save_layer_alpha`/`render_and_blend_layer` path resets
`current_state.alpha` to `1.0` while rendering the offscreen layer, meaning content is still fully
evaluated (including any `animation-tick()`-driven binding) every frame at `opacity: 0`. This
candidate reuses candidate 054's exact patch and builds the scene needed to actually measure that
specific path on Skia.

## The change

Identical to candidate `054.OPACITY_ZERO_SKIP_RENDER`'s `patch.diff` (re-verified to still apply
cleanly against current `HEAD`): an early `if opacity <= 0.0 { return
RenderingResult::ContinueRenderingWithoutChildren; }` in `internal/core/item_rendering.rs`'s
default `visit_opacity` (used by the software renderer), `internal/renderers/femtovg/itemrenderer.rs`
(additionally releasing the item's cached layer texture), and
`internal/renderers/skia/itemrenderer.rs` - all three placed before `need_layer`/`apply_opacity`
are even reached, so on the layer-needed path no offscreen layer is allocated or rendered into at
all. See `patch.diff` (23 total lines across the 3 files).

## A harness fact this candidate had to discover: no existing probe reaches the layer path

`perf_test`'s existing `probe-count`/`probe-opacity`/`probe-animate` (added by candidate 068) is a
leaf item with no children of its own, deliberately built to take the *non*-layer path (`need_layer
== false`). To reach the layer path, added `MainWindow.probe-layer-count`/`probe-layer-opacity`/
`probe-layer-animate` to `perf_test/ui/main.slint`: same idea, but each item has one
statically-declared nested child `Rectangle`, so `Opacity::need_layer` (`internal/core/items.rs:955`)
returns `true` (`opacity_child.first_child().is_some()`) - verified directly with a temporary
`std::eprintln!` in `need_layer` before benching (`need_layer probe: true` x15 for 5 probe-layer
items over 3 frames; `false` x15 for the equivalent `probe-count` leaf items), then removed before
building the real measurement binaries. Placed in its own on-screen region (`x: 700px..`, `y:
32px..`) so it doesn't overlap `probe-count`'s grid and both can be used independently or
together. Wired through `backend.rs`/`bench.rs`/`skia_headless.rs` the same way `probe-count` was;
`--frames` (the winit path) still doesn't wire any probe fields, matching the existing design
(only `--bench` and `--skia-bench` need them). Full copies of the touched files are in
`perf_test_fixes/` for reproducibility, per this workspace's usual model (`perf_test/` itself is
untracked in the main repo).

## What was tried

Bench: `perf_test --skia-bench --mode list --count 3 --probe-layer-count 1400
--probe-layer-opacity 0 --probe-layer-animate --frames 20` (headless, real Skia via its own
`SoftwareSurface`, no display/GL needed). 1400 items sized/positioned to fit fully inside the
default 1280x860 window (avoiding any subtree-bounds-pruning confound from off-viewport content).

| Variant | scenario | single-core p50 (`taskset -c 3/7`) | multi-core p50 (unpinned) |
|---|---|---|---|
| baseline | `--probe-layer-count 1400 --probe-layer-opacity 0 --probe-layer-animate` | 1.944 ms | 1.211 ms |
| patched | same | 1.430 ms (**-26.4%**, 8/9) | 0.919 ms (**-24.1%**, 9/9) |
| baseline (repeat) | same | 1.206 ms | 1.196 ms |
| patched (repeat) | same | 0.927 ms (**-23.1%**, 9/9) | 0.924 ms (**-22.7%**, 9/9) |
| baseline | `--probe-layer-opacity 1` (sanity: gate never fires) | 3.978 ms | - |
| patched | same | 3.952 ms (-0.7%, 6/9 - noise) | - |
| baseline (swapped position, sanity check) | `--probe-layer-opacity 0` | 1.186 ms (now "B") | - |
| patched (swapped position) | same | 0.911 ms (now "A", **still faster**) | - |

`RUNS=9`, `tools/paired_skia_headless.sh`, run twice per configuration (single-core and
multi-core) per `MEASUREMENT.md`; the swapped-position run rules out a simple binary-ordering
artifact (the win holds regardless of which binary is passed as "A").

Software renderer cross-check, matching candidate 054's own scenario (`--bench --mode rects
--count 50000 --opacity 0 --animate`, `taskset -c 7`, manual alternating A/B/A/B since
`tools/paired.sh` has no `TASKSET` support):

| Variant | single-core p50 | multi-core p50 (`tools/paired.sh`, unpinned) |
|---|---|---|
| baseline | 168.49 ms | 165.30 ms |
| patched | 159.80 ms (**-5.2%**, 9/9) | 161.27 ms (-2.4%, 7/9 - not consistent) |
| baseline (repeat) | 171.43 ms | 163.79 ms |
| patched (repeat) | 162.23 ms (**-5.4%**, 9/9) | 158.26 ms (-3.4%, 6/9 - not consistent) |

FemtoVG cross-check (`--frames --renderer femtovg` under `xvfb-run`/`LIBGL_ALWAYS_SOFTWARE=1`,
`tools/paired_femtovg_later_frames.sh`, same `rects --count 5000 --opacity 0 --animate` scenario
as candidate 054): baseline 50.76 ms, patched 50.57 ms (-0.4%) - within noise, no paired win-count
available from this script (see `README.md`).

Allocation counts (deterministic): identical between baseline and patched in the Skia probe-layer
scenario (7802/frame both), and in the femtovg probe-layer scenario (942/frame both) - Skia's/
femtovg's offscreen-layer allocation is not visible to the Rust global allocator (GPU/native
texture and picture-recording buffers), so unlike candidate 068's leaf-case finding, the
allocation counter cannot be used as independent evidence of the skip here; the wall-clock numbers
above are the only evidence for the Skia layer-path win. For the software `rects/50000` scenario,
allocations dropped as expected (1 300 212 -> 1 250 212, matching candidate 054's own -1
allocation/rectangle/frame finding).

## Result

**A strong, reproducible win on Skia specifically for the offscreen-layer path** (`Opacity::need_layer
== true`): -22.7% to -26.4%, single-core and multi-core, reproduced twice each (8-9/9 or 9/9 paired
wins every time), and confirmed not to be a binary-position artifact by swapping which binary is
passed first. This is the scenario candidate 068 predicted but could not measure (its own gate is
a no-op there by construction) - candidate 054's `visit_opacity` early-out is what actually closes
it, by skipping the layer setup entirely instead of gating individual draw calls inside it. At
`opacity: 1` (gate never fires) the delta is within noise (-0.7%, 6/9), confirming no regression on
the unaffected path.

**Software renderer: a real but much smaller effect, single-core only.** -5.2%/-5.4% single-core
(9/9 both times, `taskset`-pinned), but the same scenario shows *no consistent* multi-core result
(-2.4%/-3.4%, 6-7/9, "NOT consistent" per `paired.sh`'s own threshold) - matching candidate 054's
original characterization of this renderer/scenario ("every paired comparison landed between 2/9
and 7/9"). See "Why the result looks the way it does" below for a related, larger-magnitude but
unrelated effect found and ruled out while investigating this.

**FemtoVG: no measurable difference** (-0.4%), matching candidate 054's own femtovg finding at a
similar scale (5000 items).

## Does it change behavior/output?

- **Bit-exact?** Yes. `tools/pixelcheck.sh` (software renderer, `--bench`) at
  `--probe-layer-opacity 0`, `0.5`, and `1` all report "pixel-identical outside the noise floor"
  (0 differing pixels outside noise, `RUNS=5`, `--mode list --count 3 --probe-layer-count 1400
  --probe-layer-animate`).
- **Is this a breaking change?** No - invisible content (`opacity: 0`) already produced no visible
  pixels; this only skips otherwise-unobservable side work (property evaluation, offscreen layer
  allocation/render/composite) and stops `animation-tick()`-driven bindings behind `opacity: 0`
  from re-arming the animation driver forever.
- `cargo test -p i-slint-core --all-features`: **319 + 71 + 1 passed, 0 failed**, identical before
  and after the patch.
- `cargo check -p i-slint-renderer-femtovg`: clean, before and after.
- `RENDERER=skia cargo build --profile rdebug --bin perf_test --features skia` (the actual build
  used for every Skia number above): succeeds cleanly, before and after.
- `cargo check -p i-slint-renderer-skia` (default features, no `perf_test`): fails with a
  **pre-existing, unrelated** error (`softbuffer`/`backend_dispatch` macro-exhaustiveness errors
  deep in a `winit` transitive dependency) - same failure candidate 054 already noted, predates
  this patch, orthogonal to `itemrenderer.rs`.

## Why the result looks the way it does

Same mechanism as candidate 054's own analysis, now actually measured on the path it predicted by
code reading: Skia's `save_layer_alpha`/`render_and_blend_layer` resets `current_state.alpha` to
`1.0` for the duration of rendering into the offscreen layer, so every one of Skia's own
per-draw-method alpha gates (candidate 068) is unreachable in the "not visible at all" case - the
*only* way to skip that work is to never enter `need_layer`'s branch in the first place, which is
exactly what candidate 054's early return in `visit_opacity` does. The allocation counter (candidate
068's evidence for the leaf case) can't confirm this independently here because Skia's layer/texture
allocations happen on its own (non-Rust-tracked) side; the wall-clock numbers are the only signal,
which is why this candidate leaned on `paired_skia_headless.sh`'s paired-run discipline (9/9,
repeated twice, position-swap sanity check) rather than a single measurement.

**A confound found and ruled out along the way, worth recording so a future candidate doesn't
re-discover it the hard way:** an early attempt to measure the software renderer used a *much*
smaller/artificial scene (`--mode list --count 3` with only the probe items as real content) to
isolate the per-item `Opacity` cost. That scene showed the *patched* binary reproducibly ~25%
*slower* than baseline - stable to the microsecond across 5+ separate process launches, `taskset`-
pinned, on both binary orderings (so not a "two performance modes" artifact per `paired.sh`'s own
header comment either). Investigating: a purely cosmetic, semantically-inert edit to the exact same
function (`let opacity = opacity_item.opacity(); self.apply_opacity(opacity);` - functionally
identical to the unpatched code, still called for zero probe items in this scene) produced yet a
*third*, different, equally stable result (~16% *faster* than baseline). Since a no-op edit and a
real behavioral edit both produced large, stable-but-different deltas on a scene where the
`Opacity`-handling code is barely exercised at all (a `ScrollView`'s own scrollbar-fade `Opacity`
is the only user, and it's not the item under test), this is compiler-codegen/inlining sensitivity
in a hot, heavily-monomorphized generic rendering path (`item_rendering.rs`), not a property of
either patch's actual logic. Switching to the same larger, more representative scene candidate 054
originally used (`rects`, 50 000 real `Opacity`-wrapped items) made this artifact disappear and
produced the small, single-core-consistent, multi-core-inconsistent result reported above instead.
**Recorded in `IDEAS.md`** as a caution for any future candidate benching `item_rendering.rs`
changes with a tiny/synthetic scene on the software renderer.

## What's not done / open questions

- **The patch itself is unchanged from candidate 054's** - this candidate's contribution is
  purpose-built harness coverage (`probe-layer-*`) and the Skia-specific measurement candidate 054
  and 068 both could not produce. No alternative implementation was tried.
- Not tested against a real GPU-backed Skia surface (only `--skia-bench`'s `SoftwareSurface`, no
  display/GL/Vulkan) - matches this workspace's existing constraint.
- FemtoVG's layer-path number (probe-layer scenario, not just the classic `rects/5000`) was
  measured twice (+5.2% noisy, then 0.0%) but not repeated enough times to rule out the same
  scene-shape sensitivity found on the software renderer; the `rects/5000` cross-check (matching
  candidate 054 exactly) is the more trustworthy femtovg number reported above.
- `probe-layer-count`'s nested child is a plain `Rectangle` with no content of its own (chosen to
  minimize confounds); a real-world `need_layer == true` subtree (drop-shadow sibling, `visible:`
  binding) might have a different fixed per-item cost profile than this synthetic minimal case -
  not cross-checked against `rect-mode.slint`'s own opacity-capable elements (which candidate 068
  found always hit `need_layer == true` today) directly on Skia.
