# 65 - A per-item geometry `PropertyTracker` to let the dirty-region walk skip clean items

IDEAS.md's top backlog entry named the dirty-region walk (`PartialRenderer::compute_dirty_regions`,
candidates 032/033) as the one of the three remaining subtree-bounds consumers that "cannot skip
visits at all without breaking redraw scheduling for culled items" - the walk itself is what keeps
geometry dependencies fresh, so it can't just read a cache and bail the way candidates 049-051 did
for render/hit-test. This candidate looks for a design that respects that constraint (it must keep
registering *some* dependency so a culled item's future geometry change still schedules a redraw)
while still skipping the per-item work for items that provably didn't change.

## What was tried

The property system already has a purpose-built primitive for exactly this shape of problem:
`PropertyTracker::evaluate_if_dirty` - "call the closure only if dirty, but register a dependency
to the current binding either way" (used elsewhere in the codebase for the same
recompute-only-when-needed pattern: `item_rendering.rs`, `menus.rs`, `component_container.rs`,
`items/path.rs`, `items/input_items.rs`).

Each item's cache entry (`PartialRenderingCachedData`) gets a new `geom_tracker: Pin<Box<PropertyTracker>>`,
created once (default state: dirty) alongside the entry. `compute_dirty_regions`'s previously
unconditional `CachedItemBoundingBoxAndTransform::new::<T>(...)` call (which reads geometry
x/y/width/height, `clips_children()`, and `children_transform()` - all tracked reads, per
candidate 033's finding that these were 26% of the walk's cost) is now wrapped in
`geom_tracker.evaluate_if_dirty(...)`:

- If the tracker is dirty (first visit, or something it previously read changed): run the full
  computation exactly as before, which also re-establishes the tracker's own subscriptions for
  next time.
- If clean: skip the computation entirely and reuse the cached `CachedItemBoundingBoxAndTransform`
  unchanged. `evaluate_if_dirty` still calls `register_as_dependency_to_current_binding()` on the
  geom_tracker itself (a cheap operation, no property reads), which is what keeps a *future*
  change to any of those properties correctly scheduling a redraw even while the item stays culled
  - the same mechanism the existing `tracker` field already relies on for the render-content case
  (`partial_renderer.rs`'s own doc comment: "register dependencies ... on the non-dirty property
  trackers").

The item's own `bounding_rect()` call (already read `evaluate_no_tracking` today, e.g. `BoxShadow`
extending it from `blur`/`spread`/`offset`, which are therefore *not* part of `geom_tracker`'s
dependency set) stays inside the wrapped closure, i.e. it is *not* refreshed on a clean-tracker
skip. This is a known, narrow, characterized limitation - see "Does it change behavior/output?".

Full patch (also touches `filter_item`'s destructuring pattern, unrelated logic unchanged) in
`patch.diff`. `perf_test`, software renderer (`--bench`), `rdebug` profile, paired interleaved
runs (`tools/paired.sh`, RUNS=9, FRAMES=20), baseline = plain `master` (`cdb6c43f7`):

| scene / row style | scenario | single-core (`taskset -c 2`) | multi-core |
|---|---|---|---|
| `scroll` `layout` (5 items/row, `Text` children) | scrolling | **+9.7% regression (9/9)** | - |
| `scroll` `layout` | static (`--scroll-step 0`) | **+11.5% to +13.4% regression (9/9, reproduced twice)** | **+13.8% regression (9/9)** |
| `scroll` `flat` (4 items/row, `Text` children, explicit position) | scrolling | **+10.1% regression (9/9)** | - |
| `scroll` `bare` (1 item/row, no children) | scrolling | **-12.3% (9/9)** | **-14.4% (8/9, consistent)** |
| `scroll` `clipped` (`layout` + `clip: true`) | scrolling | **-11.3% (9/9)** | - |
| `list` 100000 (virtualized `ListView`) | scrolling | +0.0%, **not consistent (6/9)** | - |
| `rects` 3000 (all visible) | static-ish (moving primitives) | -2.8%, **not consistent (7/9)** | - |

Pixel correctness (`tools/pixelcheck.sh`, RUNS=5): pixel-identical outside the noise floor on
`scroll` (`layout`/`bare`/`clipped` x scrolling, `layout` x `--mutate push/insert-front/rotate/changed`,
`layout` x `--resize-step 5`), `list`, `rects` (with and without `--shadow`), `text`, `nest`,
`images`, and `rects --animate`.

## Result

A genuine, reproducible **mixed** result - not a wash, an actual split by scene composition:

- **Rows built from a leaf item with no children (`bare`) or whose children are already pruned by
  an existing mechanism (`clipped`'s `clip: true` `SkipChildren` shortcut) win consistently**
  (-11% to -14%, single- and multi-core, 8-9/9 paired).
- **Rows whose children (`Text` items) are fully visited every frame regardless of visibility
  (`layout`, `flat`) regress consistently** (+9.7% to +13.8%, 9/9 paired, reproduced on a repeat
  run), including in the fully **static** case (`--scroll-step 0`, nothing moving at all) where
  the tracker should have the most to skip.
- `list` (virtualized) and `rects` (all items visible, no pruning applies either way) are washes,
  as expected - there's nothing this change could help or hurt there.

The static-case regression is the most informative data point: with nothing moving, the geom
tracker for a `layout`/`flat` row's `Text` children should be clean on every frame after the
first, i.e. the skip path should fire close to 100% of the time - yet the patched binary is still
~12% *slower* there than baseline, which read all four geometry properties fresh every frame with
no tracker at all. So the fixed per-item cost of touching the new heap-allocated tracker (an
`is_dirty()` check plus a `register_as_dependency_to_current_binding()` call, both requiring a
thread-local lookup and a pointer chase through `Pin<Box<PropertyTracker>>`) costs *more*,
per item, than the four property reads it replaces - for these item types.

Allocations tell a different story from wall time, which is itself worth recording: on
`scroll --row-style layout --scroll-step 0 --count 100000`, later-frame allocations drop from
700 506 to 400 386 (-43%), consistent with candidate 033's finding that geometry-read dependency
registration is a real allocation source - but wall time still regresses (+11-14%). Fewer
allocations did not translate into less time here.

## Does it change behavior/output?

- **Bit-exact?** Yes, on every scenario checked (see pixel-correctness list above).
- **Characterized limitation (not observed to cause a difference in any scene tested, but real by
  construction):** `geom_tracker` does not include the properties an item's own `bounding_rect()`
  reads beyond `geometry` (e.g. `BoxShadow`'s `blur`/`spread`/`offset`/`inset`, read via
  `evaluate_no_tracking` inside `CachedItemBoundingBoxAndTransform::new`, exactly as in unpatched
  master). Unpatched master still gets a fresh value for these every frame regardless, because it
  unconditionally recomputes `new_geom` for every item every frame; this patch only recomputes
  them when `geom_tracker` is dirty. Net effect: if a culled `BoxShadow`'s `blur`/`spread`/`offset`
  changes while its `x`/`y`/`width`/`height`/`clip` stay static, **and** a redraw happens anyway
  for an unrelated reason, unpatched master's dirty-region walk would still pick up the new shadow
  extent that frame; this patch would not, until something changes the item's own tracked geometry
  again. Not exercised by any `perf_test` scene (`--shadow` in the `rects` mode is static, and
  `rects`' items are never culled) - flagged here rather than silently accepted, per the
  candidate 051 precedent for a similarly narrow, untested staleness window.
- **Is this a breaking change?** No public API changes. `geom_tracker` and the restructured match
  arm are private to `partial_renderer.rs`.
- No project test suite was run against this patch (measurement-only session); the correctness
  evidence is the pixel-comparison harness above plus the reasoning in "Why" for the one
  characterized limitation.

## Why the result looks the way it does

`compute_dirty_regions` runs inside `WindowInner::draw_contents`'s single top-level
`redraw_tracker.evaluate_as_dependency_root(...)` call (candidate 032's finding): every item's
geometry read today registers *directly* as a dependency of that one tracker, a flat, one-level
structure. This patch inserts a second tracker per item in between - `geom_tracker`, itself
registered as a dependency of the redraw tracker, and in turn depending on the item's own
properties. That indirection is exactly what `evaluate_if_dirty` is designed to make cheap to
*check*, but each item still needs a heap-allocated `Pin<Box<PropertyTracker>>` (with its own
`DependencyListHead`) that the walk must dereference and touch (a thread-local access plus a
linked-list insert) on every visit, clean or not - a genuinely new, permanent per-item cost that
did not exist before. For an item type whose baseline geometry reads were already cheap and
whose value is stable frame to frame (`bare`'s single `Rectangle`, or anything a `Clip` already
prunes from the walk before it's reached), removing the reads outweighs adding the tracker touch.
For `Text` children under a `HorizontalLayout` row - a more complex binding graph, and one where,
per candidate 013, `Text::layout_info` and hence the row's own width/height involve shaping-derived
values - the tracker touch's fixed cost is not recovered, and the case that should have benefited
most (the fully static scene) is the one measured to regress most consistently.

This is a novel design not previously attempted in this workspace (032/033/049/050/051 all
concluded a coarser dependency could only reach the ~26% registration share, not the visit itself,
and did not try wrapping the whole per-item recompute in its own tracker) - and the result answers
the open question candidate 032 left ("this collapses into the subtree-bounds prize") negatively
for this specific mechanism: a per-item `PropertyTracker` is not free enough to prune this
particular O(items) walk.

## What's not done / open questions

- **The item-composition dependency (Text-bearing rows lose, leaf/already-pruned rows win) was
  not fully root-caused beyond "the tracker touch has a real fixed cost that a plain `Rectangle`'s
  cheap reads don't amortize but a `HorizontalLayout` row's `Text` children's don't either" - a
  `perf`/allocation-profiler pass was not run in this session (see README.md's environment notes on
  `perf_event_open` being blocked here) to attribute the fixed cost more precisely (TLS access
  overhead vs. cache-miss from the extra heap indirection vs. something specific to `Text`'s own
  binding graph).
- **A version without the heap allocation** (e.g. storing `PropertyTracker` inline in the cache
  slab entry rather than `Pin<Box<...>>`, if `PropertyTracker`'s `!Unpin`ness can be satisfied
  in-place inside the `slab::Slab`) was not attempted; the per-item cache-miss from a separate heap
  object is one of the two suspected causes above and might change the picture for the `layout`/
  `flat` case specifically.
- **The characterized `BoxShadow`-adjacent limitation** above was not engineered around (e.g. by
  splitting `bounding_rect()`'s untracked recompute out so it always runs regardless of
  `geom_tracker`'s state, matching master's exact per-frame semantics) - doing so was sketched but
  not implemented, since the core mechanism was already shown not to pay off before that refinement
  would have mattered.
- **Layout's `Text::layout_info` pruning** (the other open item from the same backlog entry) is
  untouched - still needs its own design per candidate 013's finding (width isn't derivable
  without shaping).
- Only the software renderer (`--bench`) was measured, same limitation as candidates 049-051 (no
  GPU display available this session).

## Reproduce

```sh
# from ___OPTIMIZATIONS___/, with pt_base (plain master) and pt_patched (this patch) built
RUNS=9 FRAMES=20 taskset -c 2 bash tools/paired.sh /tmp/pt_base /tmp/pt_patched -- \
    --mode scroll --count 100000 --row-style layout --scroll-step 0
RUNS=9 FRAMES=20 taskset -c 2 bash tools/paired.sh /tmp/pt_base /tmp/pt_patched -- \
    --mode scroll --count 100000 --row-style bare
RUNS=5 bash tools/pixelcheck.sh /tmp/pt_base /tmp/pt_patched -- --mode scroll --count 20000 --row-style layout
```
