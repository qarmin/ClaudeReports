# 49 - A cached per-frame subtree bounding box lets the render traversal skip culled subtrees

Candidates 012, 013, 015, 032 and 033 all point at the same missing primitive from different
traversals (render/layout/hit-test/dirty-region): nothing in the tree knows a subtree's bounds
without visiting every descendant, so `render_item_children` unconditionally recurses into a
culled item's children even though they cannot be visible either. Candidate 012 sized the render
traversal's prize at up to 5.2x and named the design constraint: pruning has to be free per item,
or the fix pays for (part of) itself, the way `clip: true`'s synthetic `Clip` item already does.

This candidate implements the render-traversal slice of that design (option 1 from candidate 012:
"cache the subtree bounds per item next to the existing per-item rendering cache, refreshed
whenever the item's own geometry is re-evaluated") as a real patch, deliberately scoped to just
that one traversal rather than attempting hit-test/layout/dirty-region in the same change (see
"What's not done" below for why those need a different mechanism).

## What was tried

Mechanism: `PartialRenderer::compute_dirty_regions` already visits every item once per frame and
refreshes each item's own bounding rect/transform in its cache entry (this is candidate 032's
"unavoidable" walk - it also registers the geometry dependencies that schedule the next frame for
culled items, so it cannot be removed). Immediately after that walk, a new second post-order pass
(`compute_subtree_bounds`) recomputes, for every item, the union of all its descendants' bounding
rects (in the item's own local frame) purely from the cache entries the first pass just wrote - no
new property reads, no new dependency registrations. `render_item_children` then asks a new
`ItemRenderer::subtree_can_be_skipped` (default `false`, only overridden by `PartialRenderer`)
before recursing into a culled, non-clipping item's children; if the cached subtree bound doesn't
intersect the current clip/dirty region, the recursion (and everything under it, including a
`clip: true` row's own `Clip` item and its bookkeeping) is skipped entirely via the existing
`RenderingResult::ContinueRenderingWithoutChildren`.

Because the second pass is a full second O(items) traversal (not incrementally memoized across
frames - see "What's not done"), it has a real, measured fixed cost that only pays off when there
is something to prune. `scroll` at 100 000 rows, software renderer, `taskset -c 2` (single-core)
and unpinned (multi-core), paired interleaved runs (`tools/paired.sh`, RUNS=9, FRAMES=20):

| row style (`--row-style`) | scenario | single-core | multi-core |
|---|---|---|---|
| `layout` (5 items/row) | scrolling | -1.6% (9/9) | -1.0% (8/9) |
| `layout` | static (`--scroll-step 0`) | **-16.0% (9/9)** | **-16.1% (9/9)** |
| `flat` (4 items/row, explicit position) | scrolling | +1.5%, **not consistent (4/9)** | - |
| `flat` | static | **-16.3% (9/9)** | - |
| `clipped` (`layout` + `clip: true`) | scrolling | +2.9% (consistent, 1/9 B faster) | - |
| `clipped` | static | **-15.3% (9/9)** | **-15.3% (9/9)** |
| `bare` (1 item/row, no children) | scrolling | **+16.6% regression (9/9)** | - |
| `rects` 3000 (all visible, no pruning to do) | static-ish (moving primitives) | **+7.1% regression (9/9)** | - |
| `list` 100000 (virtualized `ListView`) | scrolling | -4.7% (9/9) | - |

Pixel correctness (`tools/pixelcheck.sh`, RUNS=4-5): pixel-identical outside the noise floor on
every row style x scenario above, plus `list`, `rects`, `text`, `nest`, `images` spot-checked
separately.

## Result

Real but narrower than candidate 012's headline 5.2x, and direction-dependent:

- **Static scenes win clearly** (-15 to -16%, 9/9 paired, both single- and multi-core): with
  nothing changing between frames, `compute_dirty_regions`'s existing walk is already cheap
  (nothing is dirty), so the new second pass's cost is small relative to the render descent it
  lets `render_item_children` skip for ~99% of the 100 000 off-screen rows.
- **Scrolling scenes are a wash or a small loss**: while scrolling, most rows' cached geometry
  changes every frame (they're inside a moving `Flickable` viewport in local coordinates - see
  "Why" below), so `compute_dirty_regions` marks most of the tree dirty and does most of its own
  work regardless; the second pass still runs in full every frame, and there its cost is not offset
  by an equivalent render-side saving. `flat` scrolling was measured explicitly inconsistent (4/9),
  i.e. genuinely a wash on this hardware, not a small win obscured by noise.
- **`clipped` scrolling regresses slightly (+2.9%, consistent)**: the existing `clip: true`
  mechanism already prunes render descent on its own (candidate 012's "escape hatch"); this
  candidate's extra pass adds cost on top without much left to prune while scrolling.
- **Scenes with nothing to prune regress** (`bare` +16.6%, `rects` +7.1%, both consistent 9/9):
  the second pass still walks the whole tree even when there are no off-screen subtrees (`bare`
  has no children per row at all) or nothing is culled (`rects`, all primitives on screen). This is
  pure added cost with no compensating saving - the "spends part of the prize on itself" pattern
  candidate 012 found in `clip: true`, in a different traversal.
- **`list` (virtualized) improved slightly** (-4.7%, 9/9): unexpected, since a `ListView` only ever
  materializes visible rows; not investigated further (small effect, may be a lower up-front cost
  in `filter_item`'s call path rather than pruning per se).

## Does it change behavior/output?

- **Bit-exact?** Yes, on the scenes checked (`scroll` 100000 across `layout`/`flat`/`clipped`/`bare`
  row styles x scrolling/static, plus `list` 100000, `rects` 3000, `text` 2000, `nest` 8500, `images`
  500). `tools/pixelcheck.sh` reported 0 px outside the noise floor in every case.
- **Is this a breaking change?** No public API changes; the new `ItemRenderer::subtree_can_be_skipped`
  trait method has a default implementation (`false`), so every renderer other than
  `PartialRenderer` (femtovg's/skia's own non-partial paths, the plain software renderer without
  partial rendering, any external `ItemRenderer` impl) is unaffected by construction.
- No project test suite was run against this patch (not integrated into the workspace's CI in this
  session); the correctness evidence here is the pixel-comparison harness described above plus the
  reasoning in "Why" for why the computed bound cannot be an under-approximation.

## Why the result looks the way it does

`compute_subtree_bounds` deliberately reuses only data `compute_dirty_regions` already wrote this
frame (each item's `bounding_rect`/`transform` in its parent's frame), so it registers no new
property dependencies and cannot itself introduce the redraw-scheduling bug candidate 032 found
when the dirty-region walk was skipped outright - it runs unconditionally, every frame, in
addition to that walk, not instead of it. That is also exactly why it costs what it costs: it is a
second full traversal, just a cheaper one (no property reads, no `PropertyTracker`, no allocation),
so its net effect is positive only when the render descent it prunes is worth more than itself -
which is a property of the scene (how much of the tree is off-screen), not of the traversal.

One item type never reaches the new skip path at all: `Clip` items always call their own `render()`
regardless of `do_draw` (`item.as_ref().clips_children()` short-circuits to the `render()` branch in
`render_item_children`), and `Clip::render()` already does its own children-visibility decision.
The bound computed for a `Clip`'s *ancestor* is safe even though the `Clip`'s own descendants may
have stale cache entries in a frame where the `Clip`'s region was empty (`compute_dirty_regions`
skips refreshing those, per its own documented behavior) - a stale entry can only make the
ancestor's computed bound too large, never too small, so this can cost an occasional missed
pruning opportunity but never an incorrect skip.

## What's not done / open questions

- **Hit-testing, layout and the dirty-region walk itself are untouched.** Candidates 015, 013 and
  032/033 identified the same missing primitive in those three traversals; extending
  `subtree_bounds` to them needs its own design work per traversal (hit-testing has no `filter_item`-
  equivalent short-circuit point today; the dirty-region walk is the one place candidate 032 showed
  cannot simply skip visits without breaking redraw scheduling for culled items). Out of scope for
  this candidate.
- **The dominant remaining cost is the unconditional second traversal.** The natural next step -
  making `compute_subtree_bounds` incremental (skip recomputing a subtree whose own dirty state
  says nothing inside it changed) - was not attempted here. It is exactly the harder, correctness-
  sensitive half of candidate 012's option 1 ("cached, invalidated only when needed" rather than
  "recomputed every frame"), and getting the invalidation wrong risks the same class of staleness
  bug candidate 032 flagged for the dirty-region walk. This is the most promising follow-up: it
  should turn today's "static scenes win, scrolling scenes wash" split into a win in both cases,
  since the recompute cost while scrolling is exactly what an incremental version would avoid.
- **Compiler-provable bounds (candidate 012's option 2) were not attempted.** That remains a larger,
  separate design task (static/interval analysis of `x`/`y`/`width`/`height` expressions), which
  would give the "zero per-item runtime cost" version of this optimization instead of the
  "cheap-but-nonzero" one implemented here.
- Only the software renderer (`--bench`) was measured; `PartialRenderer` wraps femtovg/skia too
  (any renderer using partial rendering), but this session had no GPU display available to check
  those paths (see `README.md`'s environment notes).

## Reproduce

```sh
# from ___OPTIMIZATIONS___/, with pt_base/pt_patched built per this candidate's justfile
RUNS=9 FRAMES=20 taskset -c 2 bash tools/paired.sh /tmp/pt_base /tmp/pt_patched -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 0
RUNS=5 bash tools/pixelcheck.sh /tmp/pt_base /tmp/pt_patched -- --mode scroll --count 20000 --row-style layout
```
