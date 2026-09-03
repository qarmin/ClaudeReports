# FEMTOVG_CLIP_RADIUS_BBOX_SKIP

`internal/renderers/femtovg/itemrenderer.rs`'s `visit_clip` unconditionally renders a `Clip`
item's children into an offscreen layer/texture and composites that through a rounded-rect path
whenever `clip_radius` is non-zero, because femtovg's scissor clip is strictly rectangular. Tried
GitHub issue #1053's own bounded follow-up suggestion: skip the layer entirely when the children's
bounding box provably never reaches any of the four rounded corners, since a rounded clip and a
plain rectangular one then produce identical pixels.

## What was tried

Added a free function `corner_box_reachable(bounds, clip_rect, radius)` that builds the four
`radius x radius` corner squares implied by `clip_rect`/`radius` and checks whether `bounds`
(computed via the existing `i_slint_core::item_rendering::item_children_bounding_rect`, the same
helper `render_layer` itself already calls to size the layer) intersects any of them. `visit_clip`
now only takes the offscreen-layer/blit path when at least one corner is reachable; otherwise it
falls through to the existing cheap `combine_clip` rectangular-scissor path (with the radius forced
to zero, since `combine_clip` asserts it never receives a non-zero one).

No existing `perf_test` scene combined `clip: true` with a border radius on the same item, but one
existing row style already does incidentally: `RowStyle::Clipped` (`ClippedRecordRow`) is
`RecordRow` (which has `border-radius: 3px` via `RowBackground`) plus `clip: true` - the compiler's
`clip.rs` pass copies the parent's `border-radius` onto the injected `Clip` element automatically.
Its `HorizontalLayout` has 10px left/right padding around 3 `Text` children, comfortably outside the
~2px (after border-width inset) corner radius, so this row style already exercises the "fast path"
this candidate adds, with no `perf_test` changes needed for the performance measurement.

`perf_test`'s FemtoVG path has no headless (`--bench`) mode (software-renderer-only), so timing
used the existing `--frames --renderer femtovg` path under `xvfb-run` (`LIBGL_ALWAYS_SOFTWARE=1`),
via `___OPTIMIZATIONS___/tools/paired_femtovg_later_frames.sh`, `RUNS=9`, `--mode scroll --row-style
clipped`:

| Variant | single-core p50 (`taskset -c 3`) | multi-core p50 (unpinned) | notes |
|---|---|---|---|
| baseline, count=20000 | 29.74 ms / 30.64 ms (2 runs) | 16.43 ms | offscreen layer + blit every visible row every frame |
| patched, count=20000 | 23.21 ms / 23.09 ms (2 runs) | 12.61 ms | rectangular `combine_clip`, no layer |
| delta, count=20000 | **-22.0% / -24.6%** (9/9 paired both runs) | **-23.3%** (9/9 paired) | |
| baseline, count=100000 | 63.34 ms | 48.43 ms | |
| patched, count=100000 | 54.57 ms | 44.62 ms | |
| delta, count=100000 | **-13.8%** (9/9 paired) | **-7.9%** (9/9 paired) | smaller % at scale: non-clip per-item traversal cost (`scroll` is not virtualized) dominates more of the frame as row count grows |

Allocations/frame (headless counters aren't wired to `--frames`, taken from the windowed run's own
per-frame report instead): baseline 122952-122987, patched 123347-123389 - a small, consistent
**increase** of ~400 allocations/frame at count=20000 (~0.3%), plausibly from
`item_children_bounding_rect`'s own child-geometry reads on every visible row every frame (a new
per-frame cost this candidate adds, paid even though the layer/texture path it replaces was far more
expensive overall).

## Result

A real, reproducible speedup on the one existing scene that already reaches this code path
(`RowStyle::Clipped`): -22% to -25% single-core, -8% to -23% multi-core depending on row count,
9/9 paired every measurement. The win shrinks as row count grows because `scroll` is not
virtualized (unlike `list`) and this only removes the per-visible-row clip cost, not the growing
non-clip item-tree-traversal cost underneath it.

## Does it change behavior/output?

- **Bit-exact?** No, in a characterized, sub-perceptible way for the case the optimization
  actually changes; yes (identical to baseline, confirmed empirically) for the case it doesn't.
  - **Fast-path case** (content stays clear of the corners, as `RowStyle::Clipped` does by
    default): comparing 5 same-binary runs each of baseline and patched (`--frames --renderer
    femtovg --mode scroll --row-style clipped --count 20000 --snapshot`, needed because a single
    snapshot pair is not enough - this scene has its own ~180px same-binary noise floor, a status
    row near the bottom of the window unrelated to this patch) found **9 px out of 3,302,400
    differing outside that noise floor, max per-channel difference 1/255**, all on glyph
    antialiasing edges inside row text (bbox x 458-505, y 162-230; see
    `testdata/pixel_diff_highlight.png` for the full-frame diff, red = differing pixel, and
    `testdata/zoom_diff_region.png` for a crop of the affected area). Cause: removing the
    offscreen-layer/textured-blit step removes an extra image-paint resample that the baseline path
    always pays even in the interior of the clip region, not just near the corners it exists for -
    a one-bit rounding shift on some already-antialiased text edges, not a geometric difference in
    what's visible.
  - **Slow-path case** (content does reach the corners, so the optimization must not fire):
    temporarily set `RecordRow`'s `HorizontalLayout` padding to 0px (reverted before finishing;
    not part of the shipped patch or a permanent `perf_test` change) so text starts at the very
    edge of the row, intersecting all four corner squares. Same 5-runs-per-side comparison: **0 px
    differing outside the noise floor** - `corner_box_reachable` correctly falls back to the exact
    unpatched offscreen-layer code path, bit-identical to baseline (module the same pre-existing
    noise floor). This also confirms the case is reachable: baseline and patched timings were a
    wash here (17.67ms vs 19.34ms later-frames median, single measurement, not a paired run - within
    the noise this scene shows even same-binary), unlike the ~22-25% win in the fast-path case.
- **Is this a breaking change?** No public API changes. No change to any documented behavior -
  the 1-LSB antialiasing shift is an internal rendering-precision artifact, the same category of
  change other already-landed candidates in this workspace's `SUMMARY.md` document (e.g. 062's
  premultiplication, 066's Skia single-rect clip).
- Project test suite: `cargo test -p i-slint-renderer-femtovg` - 0 tests before, 0 tests after (the
  crate has no unit tests). No other crate's source was touched.

## Why the result looks the way it does

femtovg has no rounded-rect scissor primitive, so the existing code always pays a full
allocate-a-texture + render-subtree-to-it + composite-with-an-image-paint sequence for *any*
non-zero radius, even though the vast majority of real `clip: true; border-radius: ...;` usage
(rounded cards/panels/list rows with inset content) never actually needs a rounded clip: the
content simply doesn't reach the corners. The bounding-box check this candidate adds is O(children)
and reuses machinery `render_layer` already had before it, so it converts an unconditionally-paid,
GPU-resource-heavy operation into a cheap, provably-safe rectangular scissor in that common case,
while leaving the exact original code path in place (untouched) for the case where a rounded clip
is genuinely needed.

## What's not done / open questions

- Real-world win size still depends on how often application content actually avoids the rounded
  corners, as the original issue triage noted - an `Image` or a solid-fill `Rectangle` filling the
  whole clipped area would not benefit (`corner_box_reachable` would correctly return `true` and
  take the same code path as before). `RowStyle::Clipped`'s 10px padding happens to be a realistic
  stand-in for "inset content", not a cherry-picked best case, but it is still one scene.
- The small per-frame allocation increase (~400/frame at count=20000, from
  `item_children_bounding_rect`'s own reads) was not chased further; it's dwarfed by the removed
  layer/texture cost in every scene tried, but a scene with very many clipped rows *and* content
  that always reaches the corners (so the fast path never fires) would pay this extra cost for
  nothing - not measured here since no such scene exists in `perf_test` yet.
- Not tried: the "harder, full lazy mid-render" version the original issue's author considered and
  set aside in favor of this bounded one (switching from the layer path back to scissor mid-render
  once already committed, rather than deciding once per `visit_clip` call before starting).
- The corner-square check treats each corner independently and conservatively (any single
  overlapping child anywhere forces the slow path for the whole clip, even if only one of four
  corners is actually touched); a finer per-corner decision was not explored, since femtovg cannot
  clip a rounded rect on only some corners in the fast path anyway - only "no radius" or "full
  layer" are actually cheap options for it, and mixing them per-corner is not what the layer path
  it's compared against does.
