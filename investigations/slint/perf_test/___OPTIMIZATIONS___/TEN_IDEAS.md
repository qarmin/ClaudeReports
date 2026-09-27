# Ten optimization ideas, each sized in isolation

Ideas were sized by **ablation** first - deliberately breaking the code to remove the work and
measuring what the frame does - before any of them was implemented. An ablation costs one build;
implementing something worth 0% costs an afternoon. Six of the ten measured as worth nothing, which
is the point of doing it this way round.

All numbers are medians of 5-7 runs, spreads in brackets, both binaries built in the same session.

| # | idea | sized at | verdict |
| --- | --- | --- | --- |
| 1 | `FilterModel::row_added` does `iter().skip(index)` - looks like it clones every skipped row | **0%** | dead: `ModelIterator` implements `nth`, so `skip` jumps. Confirmed flat across 25k-200k models |
| 2 | Drop the four `downcast_pin` checks that run for every *culled* item | **0%** | dead: 114.11 -> 114.71 ms on `scroll` 100 000 (worse, i.e. noise) |
| 3 | Drop the dirty-region + transform computation in `filter_item` | **0%** | dead: 114.11 -> 116.20 ms |
| 4 | One document scan instead of two in `shape_paragraphs` | **0%** | dead: typing 50 000 lines 852 -> 857 ms; document load 1.68 -> 1.69 s. The memchr pass is nothing against the shaping |
| 5 | Drop `selection_geometry`'s defensive span sort | see 7 | measured with 6 and 7 |
| 6 | One `get_current_clip()` per text draw instead of two | see 7 | measured with 5 and 7 |
| 7 | Skip the per-line selection-span lookup when nothing is selected | **-3 to -5%** on text scenes, combined with 5 and 6 | marginal; see below |
| 8 | **Skip the per-item geometry read for culled items** | **-12%** of the bare traversal | **the real one**; see below |
| 9 | **Skip `save_state`/`restore_state` per item** | **-6%** of the bare traversal | plausible, needs care |
| 10 | Batch `SortModel::row_added` insertions | **-1000%** | implemented, measured, **reverted** - see candidate 29 |

## The two that are worth something

Both were sized on `scroll --row-style bare` (100 000 items, one per row, nothing drawn) so the
measurement is the traversal and nothing else:

| ablation | frame |
| --- | --- |
| baseline (correct code) | 18.90 ms (±0.50) |
| **no per-item geometry read** | **16.67 ms (±1.68)** |
| **no `save_state`/`restore_state`** | **17.72 ms (±0.48)** |

- **8: the geometry read is ~22 ns per item.** `filter_item` calls
  `evaluate_no_tracking(|| item_rc.geometry())` for every item, but a culled item only needs the
  *origin* (to translate before recursing into children) - the size is used solely by
  `item.render(..., item_geometry.size)`, which culled items never reach. The partial renderer's
  cache already stores that origin, as `CachedItemBoundingBoxAndTransform::RegularItem { offset }`.
  **The open question is now answered**: `compute_dirty_regions` walks *every* item every frame and
  refreshes the cache entry for each (candidate 32), so the cached offset is fresh. Which also means
  the geometry is read **twice per item per frame** - once there and once here.
- **9: `save_state`/`restore_state` is ~12 ns per item.** They are needed - `translate` mutates the
  state that `restore_state` pops - but the pair could be replaced by an inverse translate after
  the recursion for the common case of an item that changes nothing else. Riskier than it sounds:
  any child that touches the clip or alpha relies on the pop.

**8 is now implemented** - see `35_cull_geometry_read/RESULT.md`. It is worth **-12%** on the same
scene, close to what the ablation predicted, but only in its second form: the obvious
implementation repeated the caller's item-kind test on the hot path and measured as a wash with a
bimodal distribution. **9 is now implemented too** - see `36_conditional_state_stack/RESULT.md`, worth **-8.2%** on the
same scene. Its "riskier than it sounds" caveat was right about *what* to check and wrong about the
answer: children balance their own state, so only an item that renders needs the stack.

## 5, 6 and 7 together: -3 to -5%, and not separable

Ablated as a group (removing the sort, the second `get_current_clip`, and the span lookup):

| | `text` 400 000 `--select-all` | `text` 100 000 |
| --- | --- | --- |
| baseline | 4.41 ms (±0.03) | 3.96 ms (±0.17) |
| all three ablated | 4.20 ms (±0.03) | 3.85 ms (±0.02) |

Outside the spreads, but 0.1-0.2 ms split three ways is not worth three separate changes to
correctness-sensitive code (the sort is documented as defensive precisely because both consumers
fail silently without it). Recorded and left alone.

## What the exercise says about the engine

Six of ten ideas were worth exactly nothing, and the two that were worth something are both in the
**per-item traversal** - the same place candidates 12, 15, 18 and 28 keep pointing at. There is no
micro-optimization left in the text or model paths that moves a frame; the remaining cost is
visiting items, and the way to win is to visit fewer of them.
