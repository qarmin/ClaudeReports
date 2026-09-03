# 32 - The dirty-region walk is a second O(total items) traversal, and past ~10 000 items it can only lose

**Status: quantified with a pixel-identical ablation. The largest single cost found in this
workspace.**

Before rendering, `PartialRenderer::compute_dirty_regions` walks **every item in the tree** and
builds a `CachedItemBoundingBoxAndTransform` for each, to work out which screen region needs
repainting. That is a full traversal on top of the rendering traversal, and nothing had measured it.

## Sizing it fairly

The obvious ablation - deleting the call - is invalid: the dirty region stays empty and the renderer
skips work, so the frame is fast for the wrong reason. Checked, and the output did differ (still 385
distinct colours, so not blank, but not the same frame either).

The fair ablation skips **only the per-item walk** and sets the dirty region to the whole screen, so
the renderer draws everything:

```rust
let _ = components;                                  // no walk
partial_renderer.dirty_region = screen_region.into(); // repaint everything
```

Output is **pixel-identical** to the baseline on `rects`, `scroll` and `text`, which is what makes
the numbers comparable.

## While scrolling, it is 59% of the frame and saves nothing

| scene, scrolling | with the walk | without | |
| --- | --- | --- | --- |
| `scroll` 100 000 | 121.83 ms | **49.35 ms** | **-59%** |
| `scroll` 100 000 `bare` | 19.28 ms | **8.21 ms** | **-57%** |
| `rects` 3 000 | 2.49 ms | 2.08 ms | -16% |
| `list` 100 000 | 2.36 ms | 2.19 ms | -7% |

Everything on screen changes every frame here, so the dirty region is the whole screen anyway - the
walk computes, at O(total items) cost, an answer it could have assumed.

## Statically, it pays for itself - until the tree gets big

Nothing changes between frames (`--scroll-step 0`), which is the case the walk exists for:

| scene (static) | with the walk | without | |
| --- | --- | --- | --- |
| `rects` 300 | **0.09 ms** | 0.87 ms | pays off 9.7x |
| `rects` 1 000 | **0.32 ms** | 1.90 ms | pays off 5.9x |
| `rects` 3 000 | **0.75 ms** | 1.98 ms | pays off 2.6x |
| `rects` 10 000 | **2.43 ms** | 2.86 ms | pays off 1.2x |
| `nest` 8 500 items | **1.01 ms** | 1.34 ms | pays off 1.3x |
| `scroll` 5 000 `bare` | **0.72 ms** | 1.54 ms | pays off 2.1x |
| `scroll` 20 000 `bare` | 2.82 ms | **2.46 ms** | **costs 1.1x** |
| `scroll` 100 000 `bare` | 17.44 ms | **8.48 ms** | **costs 2.1x** |

**The crossover is around 10 000-20 000 items.** The reason is structural: the walk's cost scales
with the **item tree**, its benefit with the **screen area**. Screens do not get bigger when a model
does, so past some tree size the walk cannot win, and a scrolling UI never lets it win at all.

## What to do about it

1. **Subtree bounds** (candidates 12, 13, 15) would prune this walk too, not just rendering and
   hit-testing. This is a *fourth* traversal with no notion of what is on screen, and the largest
   one - which makes the case for subtree bounds substantially stronger than candidate 12 estimated
   from the rendering traversal alone.
2. ~~**A cheap interim heuristic**: skip the walk and repaint the screen when the item count is
   large.~~ **Retracted - this is not correct.** See "Why the walk cannot simply be skipped" below.
3. The walk also **reads every item's geometry**, which `filter_item` then reads *again* during
   rendering (idea 8 in `TEN_IDEAS.md`, sized at -12% of the traversal). Fixing the duplication is
   worth less than not walking at all, but it is much smaller and its correctness question is now
   answered: because this pass refreshes the cache for every item every frame, the cached offset
   `filter_item` would reuse **is** fresh.

## Reproduce

```sh
# the ablation is in the write-up above; apply it to partial_renderer.rs, then:
RUNS=7 tools/matrix.sh \
  "scrolling|--mode scroll --count 100000 --row-style bare" \
  "static|--mode scroll --count 100000 --row-style bare --scroll-step 0"
```

## Why the walk cannot simply be skipped

The ablation above measures the walk's **cost** honestly - it is pixel-identical because the frame
is repainted in full. It does **not** show that the walk is skippable, and a follow-up attempt to
turn it into a real fix (an `ItemVisitorResult::Abort` once the region covers the screen, and a
self-tuning "was the last frame full-screen?" switch) ran into a blocker that makes both wrong.

**The walk has a second, load-bearing job: it is what makes the window redraw at all.**

The chain, all of it in the source:

1. `WindowInner::draw_contents` evaluates the whole render inside
   `redraw_tracker.evaluate_as_dependency_root` (`window.rs:1729`). Anything read during a frame
   becomes a dependency of that tracker.
2. `WindowRedrawTracker::notify` calls `window_adapter.request_redraw()` (`window.rs:493`). So a
   property read during a frame is exactly what schedules the *next* frame when it changes.
3. The module documentation states the walk's second job outright: *"That pass also register
   dependencies on every geometry, and on the non-dirty property trackers"* (`partial_renderer.rs:11`).
4. `filter_item` relies on that having happened, and says so: *"Query untracked, as the bounding
   rect calculation already registers a dependency on the geometry"* (`partial_renderer.rs:649`).

For an item that **is** rendered, its dependencies still reach the root without the walk, because
`do_rendering` wraps rendering in the item's own `PropertyTracker` and `PropertyTracker::evaluate`
registers itself to the enclosing binding (`properties.rs:1506-1510`). So rendered items are safe.

For an item that is **culled** - clipped away, off-screen, the 99 700 rows of a 100 000-row
`ScrollView` - `do_rendering` never runs. The walk is the *only* thing that reads its geometry, and
therefore the only thing that would notice it moving. Skip the walk and an off-screen item that
grows or moves into view no longer schedules a frame; it appears whenever something unrelated
happens to trigger one.

This is invisible in the benchmark, because the harness drives frames on a fixed schedule rather
than waiting to be asked - which is exactly the kind of bug a benchmark cannot catch and is worth
recording as such.

There is a second, smaller problem even if redraws were handled: `filter_item` reads the cached
bounding rect and only recomputes when the entry is **missing** (`partial_renderer.rs:654-659`).
Skipping the walk leaves entries *stale* rather than missing, so a moved item is culled against its
previous position. The ablation was pixel-identical on three scenes despite this; that is luck about
those scenes, not a guarantee.

### What this changes

The `-59%` number is not an available win, it is **headroom**: it is the upper bound on what
eliminating this traversal could return in a scrolling scene, and the walk can only be eliminated by
something that still registers a dependency covering the culled items - i.e. a *coarser* dependency
over a subtree rather than no dependency at all.

That is precisely the subtree-bounds design of candidates 12/13/15. So candidate 32 does not stand
as an independent fix; it **collapses into that prize and puts the first quantitative figure on it**.
Recommendation 1 is the whole recommendation.
