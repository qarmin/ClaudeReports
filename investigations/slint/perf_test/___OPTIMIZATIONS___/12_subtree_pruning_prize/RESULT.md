# 12 - What subtree pruning would be worth, measured

**Status: the prize is measured (5.2x on the pathological scene) and the design constraint is
now known. Not implemented - it is a change to the core rendering traversal.**

Candidate 07 and the user's GPU run established that allocator-side work is exhausted: with a
3.3x faster allocator the current tree gets *slower* on rect and ListView scenes. What is left is
the traversal itself. This candidate measures how much a renderer that skips the children of
culled items could win, using row variants that are all instantiated by their own `for` loop (so
switching between them adds no per-row cost).

## Measured (software renderer, `scroll` 100 000, current tree)

```sh
../target/rdebug/perf_test --bench --mode scroll --count 100000 --row-style layout|flat|clipped|bare
```

| `--row-style` | items per row | first frame | later frames | allocations/frame |
| --- | --- | --- | --- | --- |
| `layout` (Rectangle + HorizontalLayout + 3 Texts) | 5 | 3.05 s | 135.18 ms | 300 744 |
| `flat` (same 3 Texts, positioned explicitly) | 4 | 126 ms | 116.06 ms | 301 037 |
| `clipped` (`layout` + `clip: true`) | 6 | 102 ms | 99.21 ms | **601 151** |
| `bare` (Rectangle only) | 1 | 34 ms | **25.88 ms** | 200 305 |

## Two conclusions

**1. The prize is 5.2x.** A renderer that visited only the row and skipped its children when the
row is culled would land near the `bare` row: **135 ms -> ~26-30 ms** for 100 000 rows. That is
the only change measured in this whole session that moves this scene by more than ~10%.

**2. Today's escape hatch spends half the prize on itself.** `clip: true` is the only subtree
pruning the renderer has (`visit_clip` returns `ContinueRenderingWithoutChildren` for an empty
clip region), and it does work - 135 -> 99 ms - but it **doubles the allocations** (300 744 ->
601 151) because every row gains a `Clip` item with its own geometry, clip-state bookkeeping and
dependency registrations. So the pruning mechanism must be free per item: a bounds check on data
the traversal already has, not an extra item in the tree.

**3. The children's cost is not allocation.** `bare` -> `flat` adds 3 `Text` items per row and
costs +90 ms/frame with **+293 allocations total** (301 037 vs 200 305 is one per row, not one per
item). At ~275 ns per visited item, that is the `ItemRc` + `filter_item` + tracked geometry read
path, which is exactly what pruning avoids.

## Design constraint for an implementation

To skip a culled item's children, the renderer must know that no descendant draws outside the
item's own bounding rect. Options, in order of how contained they look:

1. **Cache the subtree bounds per item** next to the existing per-item rendering cache, refreshed
   whenever the item's own geometry is re-evaluated. The partial renderer already keeps a
   per-item `CachedItemBoundingBoxAndTransform` and a `PropertyTracker`
   (`internal/core/partial_renderer.rs:100-170`), so the storage and invalidation machinery for
   this exists on that path; FemtoVG has no such cache and would need one.
2. **A compiler flag per component**: "every child's `x`/`y`/`width`/`height` is provably inside
   the root's geometry" (constant or clamped expressions). Covers the common repeated-row case
   without any runtime bookkeeping, and the renderer falls back to visiting when the flag is
   absent.
3. Only prune under a clipping ancestor (a `Flickable` viewport), where an escaping child would be
   clipped anyway - still incorrect in general, because a child can be positioned outside its
   parent and inside the clip.

Option 2 is the one that would help this scene at zero per-item cost.

## Reproduce

```sh
just run                      # the table above, no display needed
just run-gpu                  # same on the real display (part of the batch, see ../GPU_BATCH.md)
```


---

## Update: the fix exists in-tree, opt-in, and is worth 24x

Candidate 44 measured `clip: true` on a row against the identical row without it, at 20 000 rows:
first frame **620 -> 25 ms**, allocations **1 641 941 -> 224 271**, steady frame **-45%**, and
paragraph shapes at startup **60 081 -> 66**.

The mechanism is the pruning this candidate asks for, already implemented:
`Clip::render` returns `RenderingResult::ContinueRenderingWithoutChildren` when the clip region is
empty (`item_rendering.rs:577`), so an off-screen clipped subtree is never visited.

So the missing piece is not the pruning. It is **knowing that a subtree is bounded by its parent
without the developer promising it via `clip`**. `clip: true` is that promise, made by hand, and the
numbers above are what it is worth.

One caveat the measurement also produced: the same promise buys much less on the *input* traversal -
**-18%, not the -50% recorded by candidate 15** (`15_.../CLIP_CORRECTION.md`) - even though it removes
about 75% of the visits there. Whatever dominates hit-testing, it is not the visit count, so a subtree
bound would not pay off equally on all four traversals.
