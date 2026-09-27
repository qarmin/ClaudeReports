# 15, corrected - `clip: true` does not halve the hit-test cost

Candidate 015 recorded: *"`ListView` +0.01 ms, `clip: true` halves it"*. Re-measured at the same size
with the current tree, `RUNS=7`:

| 100 000 rows | no input | one mouse move per frame | cost of the move |
| --- | --- | --- | --- |
| `layout` rows | 113.60 ms | 149.57 ms | **+35.97 ms** |
| `clipped` rows (same row + `clip: true`) | 74.84 ms | 104.33 ms | **+29.49 ms** |

**-18%, not -50%.**

## And -18% is itself less than the code predicts

`Clip::input_event_filter_before_children` (`items.rs:751-769`) returns `InputEventFilterResult::Intercept`
when the pointer is outside the clip, and `Intercept` maps to `forward_to_children = false`
(`input.rs:1676`). So an off-screen clipped row's children really are skipped.

The scene has 100 000 rows, each a layout plus three `Text` items - roughly 400 000 items, of which
clipping should remove the ~300 000 that are children. That is a 75% reduction in visits, and it buys
18%.

So **the input pass's cost is not dominated by the number of items it visits**, which is what
candidate 015 assumed when it derived "~98 ns per item" by dividing. That per-item figure holds for the
`bare` scene (one item per row, nothing to prune) and does not generalise to a scene with a subtree
per row.

## What it is, measured

Two hypotheses were recorded here; one is dead and the other is half the answer.

**Dead: the exit/enter stack churn.** Added `--pointer-jitter`, which moves the pointer by 0.4 px -
inside a single 34 px row - so the hit test runs in full but the set of items under the pointer never
changes and no exit/enter events fire.

| `scroll` 100 000 | frame |
| --- | --- |
| no input | 115.68 ms |
| `--pointer-jitter` (never leaves the row) | 149.24 ms |
| `--pointer-move` (crosses rows every frame) | 146.30 ms |

Identical. The cost is the traversal, not what the traversal discovers.

**Half the answer: the per-item geometry read.** `send_mouse_event_to_item` opens with
`let geom = item_rc.geometry();` (`input.rs:1652`). Ablating that to a constant - incorrect, but it
sizes the term - with `--pointer-jitter` so only the traversal is measured:

| scene | with the read | without | share of the whole frame | share of the input pass |
| --- | --- | --- | --- | --- |
| `layout` rows | 141.61 ms | 127.36 ms | **-10.1%**, 5/5 | **~43%** (14.25 of ~33 ms) |
| `clipped` rows | 102.88 ms | 93.89 ms | **-8.7%**, 4/4 | **~30%** (9.0 of ~29.5 ms) |

Both work out at **~40 ns per geometry read** - 400 000 reads in the unclipped scene (one per item),
200 000 in the clipped one (100 000 rows, read **twice** each). The remaining ~20 ms in both cases is
the visit machinery itself.

## A duplication worth removing

A clipping item's geometry is read twice per event: once by `send_mouse_event_to_item`, and again by
`Clip::input_event_filter_before_children` (`items.rs:759`), which needs only the width and height -
values the caller already has. At 100 000 clipped rows that is 100 000 redundant reads, about
**4.5 ms per mouse move**.

The caller could short-circuit it: when the position is outside `geom` and the item clips its
children, the answer is `Intercept` without asking. **Not done here**, because the two tests are not
quite the same at the boundary - `LogicalRect::contains` excludes the far edge while the `Clip` check
uses `pos.x_length() > geometry.width_length()`, which includes it. That is a one-pixel behavioural
difference on the right and bottom edges of every clipping element, and it needs deciding rather than
assuming.

## Reproduce

```sh
RUNS=7 tools/matrix.sh \
  "layout|--mode scroll --count 100000" \
  "layout + pointer|--mode scroll --count 100000 --pointer-move" \
  "clipped|--mode scroll --count 100000 --row-style clipped" \
  "clipped + pointer|--mode scroll --count 100000 --row-style clipped --pointer-move"
```
