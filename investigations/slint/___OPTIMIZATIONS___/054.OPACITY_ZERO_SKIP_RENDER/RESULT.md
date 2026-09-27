# 54 - don't visit an `Opacity` item's children when its opacity is exactly 0

From GitHub issue #6056 (see `___ISSUES___/performance/6056.md` for the full triage write-up):
all three renderers' `visit_opacity` unconditionally descend into an `Opacity` item's children
regardless of the resolved opacity value - `internal/core/item_rendering.rs:605` (default trait
method, used by the software renderer), `internal/renderers/femtovg/itemrenderer.rs:537`, and
`internal/renderers/skia/itemrenderer.rs:925` (which additionally allocates/renders into a
`save_layer_alpha` GPU layer at opacity 0). The reporter faded a `Spinner` to `opacity: 0` instead
of using `visible: false`, and observed CPU usage staying high forever because the Spinner's
internal `Path` reads `animation-tick()`, which re-arms the animation driver every time it is
*evaluated*, not just when it produces a new value.

## The change

Added the same early-out to all three `visit_opacity` implementations: if `opacity <= 0.0`,
return `RenderingResult::ContinueRenderingWithoutChildren` immediately instead of calling
`apply_opacity`/descending into children. femtovg additionally releases the item's cached layer
texture (`self.layer_cache.release(item_rc)`) on that path so it doesn't hold a stale texture
across a fade. See `patch.diff` for the exact 3 edits (2-8 lines each).

## What was tried

Only one variant (the early-out described above, applied identically to all three renderers) -
no alternative implementation was tried.

Bench: `perf_test --bench --mode rects` (headless, software renderer only - `--bench` always uses
`MinimalSoftwareWindow` regardless of `--renderer`), `--opacity 0` on every rectangle (each
rectangle is individually wrapped in an injected `Opacity` element by the compiler, since it sets
`opacity` directly), combined with `--animate` (every rectangle's `x` offset depends on
`animation-tick()` - a **geometry** property) and separately `--animate --animate-color` (every
rectangle's `background` depends on `animation-tick()` instead - a **content-only**, non-geometry
property, closer to the reported Spinner pattern where only the `Path`'s `commands` animate).

| Variant | count | scenario | single-core p50 (`taskset`) | multi-core p50 (unpinned) |
|---|---|---|---|---|
| baseline | 50 000 | geometry-driven animate | 136.01 ms | 137.83 ms |
| patched | 50 000 | geometry-driven animate | 139.20 ms (+2.3%, 2/9 - not consistent) | 138.35 ms (+0.4%, not consistent) |
| baseline | 50 000 | content-only animate (`--animate-color`) | 118.75 ms | 117.04 ms |
| patched | 50 000 | content-only animate (`--animate-color`) | 116.00 ms (-2.3%, not consistent) | 109.50 ms (single ad-hoc run, see below) |
| baseline | 100 000 | geometry-driven animate | 293-301 ms across 3 repeats | - |
| patched | 100 000 | geometry-driven animate | 287-300 ms across 3 repeats, direction inconsistent | - |

All `paired.sh` verdicts at every combination and count tried were **"NOT consistent - the medians
are not evidence"** except one 100 000-count run that landed just inside the tool's "consistent"
threshold (0/9) in one direction and a repeat of the same parameters later landing at 2/9 (not
consistent) - see "Why the result looks the way it does" below for why this one configuration
looked briefly like a small regression and why that didn't hold up under repetition.

femtovg (`--frames 15` under `xvfb-run`, real window, count 5000, content-only animate): baseline
48.7-52.6 ms, patched 47.4-49.9 ms across 7 alternating runs - overlapping ranges, no consistent
direction.

Allocation counts (deterministic, not subject to timing noise) **do** drop as expected in every
scenario: -1 allocation per rectangle per frame when the parent `Opacity`'s children are no longer
visited at all (e.g. 1 300 204 -> 1 250 204 at count 50 000, geometry-driven; 900 204 -> 850 204
at count 50 000, content-only) - confirming the intended code path is actually skipped, even where
wall-clock time doesn't show it.

## Result

**No measurable wall-clock difference on the two renderers this harness can reach (software,
femtovg)**, at the scales tried (5 000-100 000 opacity-0 animated items). Every paired comparison
was within the noise floor this project's tooling requires for a real conclusion (`paired.sh`
needs >=8/9 or <=1/9 wins to call a difference "consistent"; every run here landed between 2/9 and
7/9). The allocation-count drop confirms the skip is real and is being exercised; it is just too
small a fraction of these scenes' dominant cost (the O(total items) dirty-region walk that
`compute_dirty_regions` runs every frame regardless of opacity, and which candidates 032/033
already measured as ~59%/~26% of frame time) to surface above measurement noise.

Skia was not benchable in this session (see "What's not done" below) but is, by code reading, the
renderer this fix matters most for - see next section.

## Does it change behavior/output?

- **Bit-exact?** Yes. `tools/pixelcheck.sh` at `--opacity 0`, `--opacity 0.5`, and the default
  `--opacity 1` (i.e. no `Opacity` wrapper at all) all report "pixel-identical outside the noise
  floor" (0 differing pixels each, 5 runs/side, `--mode rects --count 2000`).
- **Is this a breaking change?** No - invisible content already produced no visible output; this
  only skips otherwise-unobservable side work (property evaluation, allocations, an offscreen
  layer render on femtovg/skia).
- Project test suite: `cargo test -p i-slint-core --all-features` - 319 + 71 + 1 passed, 0 failed,
  before and after. `cargo check -p i-slint-renderer-femtovg` and
  `cargo check -p i-slint-core --features ffi` (the ABI-mirror check from
  `projects/slint.md` section 4b) both clean. `cargo clippy -p i-slint-core --features testing
  --lib --tests` produced no new warnings on the touched file.
- `cargo check -p i-slint-renderer-skia` could not be run to completion in this environment - it
  fails to build with a **pre-existing, unrelated** error (`softbuffer`/`backend_dispatch` macro
  exhaustiveness errors deep in a `winit` dependency, nothing to do with `itemrenderer.rs`); this
  predates the patch and was not investigated further, since it is orthogonal to this candidate.

## Why the result looks the way it does

**The fix is real but the benchable renderers already had a partial version of it**, for exactly
the case the reported bug is about (a *content-only* animation, not a geometry one):

- The software renderer's `should_draw()` (`internal/renderers/software/lib.rs:2203-2207`) and
  femtovg's `global_alpha_transparent()` both gate `draw_rectangle`/`draw_path`/etc. on the
  current alpha *before* reading the item's rendering-only properties (`background`, `commands`).
  So for the Spinner's exact pattern - `path.start`/`path.progress` computed from
  `animation-tick()` and used only inside the `commands` the Path renders - software and femtovg
  were **already not calling `animation-tick()`** once alpha reached 0, before this patch, because
  `draw_path`/`draw_rectangle` return before ever touching those properties.
- **Skia has no such gate**: `internal/renderers/skia/itemrenderer.rs`'s `draw_rectangle` calls
  `rect.background()` unconditionally (first argument to `brush_to_paint`, evaluated in Rust
  before any alpha check), and `draw_path` calls `path.fitted_path_events(item_rc)` unconditionally
  too. So on Skia, before this patch, the reported bug is real and unmitigated: an
  `animation-tick()`-driven `Path`/`Rectangle` behind `opacity: 0` keeps re-arming the driver every
  frame, forever. This patch's early-out in `visit_opacity` is what actually stops that, by never
  reaching `draw_rectangle`/`draw_path` at all. Not benched here (see below), but confirmed by
  reading the exact code path Skia executes.
- What the patch *does* uniformly remove, on every renderer, is the `filter_item` call on the
  `Opacity`'s child (`internal/core/item_rendering.rs:699-711`, which reads the child's full
  geometry with no alpha gate at all) plus the `save_state`/`translate`/`restore_state` bookkeeping
  around it. That is real, confirmed by the allocation-count drop, but it is a small, fixed
  per-item cost next to the dirty-region walk that dominates these particular scenes - hence no
  visible wall-clock movement in a scene sized to stress exactly that walk.

**The one apparent regression didn't hold up:** an early single-core run at count 100 000
(geometry-driven animate) showed the patched binary ~1.4-4.3% slower, consistently in direction
across 2 of 3 repeats (including with binary order swapped, ruling out a simple position bias).
Investigation (bounded, per this project's own pitfall list): the effect disappeared entirely with
`--animate-count 1` (only one item actually animating) and with `--opacity 0` alone (no
`--animate`), and a same-parameters repeat later landed at only 2/9 (not consistent). Per-frame
timings within a single run also showed a ~20% *ramp* from frame 0 to frame 9 in both binaries on
this single pinned core (`taskset -c 5`, sustained 100% load for tens of seconds) - consistent
with core-level thermal/frequency throttling drifting during the run, a confound this project's
own tooling explicitly warns is possible and not something the two-repeat budget here could fully
rule out. Not written up as a regression given it didn't reproduce at 50 000 items, with fewer
animated items, or on the unpinned multi-core path.

## What's not done / open questions

- **Skia was not built/benched.** `skia-safe` requires compiling Google's Skia from source (heavy,
  slow, and this session hit an unrelated pre-existing build failure in a transitive dependency
  before even reaching the skia renderer crate). The skia-specific fix is the one most clearly
  justified by code reading (see above) but has no measured number here.
- **A geometry-driven animation behind `opacity: 0` is not fixed by this patch, on any renderer.**
  `compute_dirty_regions` (`internal/core/partial_renderer.rs`) reads `item_rc.geometry()` for
  every item every frame, unconditionally, regardless of any ancestor's opacity - if `x`/`y`/
  `width`/`height` themselves depend on `animation-tick()`, that call keeps re-arming the driver
  forever no matter what `visit_opacity` does, because it never gets to `visit_opacity` in the
  first place; the read happens in a separate traversal this patch doesn't touch (matching the
  original triage's note that the dirty-region pass was checked and assumed not to need touching -
  true only for content-only animations, not geometry ones). Worth a follow-up idea; added to
  `IDEAS.md`.
- Real-window (`--frames`) confirmation was only done for femtovg, under `xvfb-run`, at one count
  (5 000); not repeated at larger counts or on real (non-virtualized) hardware.
- No existing `.slint` screenshot/snapshot test was found or added that specifically covers
  `opacity: 0` with animated children; this candidate's own `pixelcheck.sh` runs are the only
  regression coverage exercised here (per this skill's scope, it does not add tests to the
  project itself).
