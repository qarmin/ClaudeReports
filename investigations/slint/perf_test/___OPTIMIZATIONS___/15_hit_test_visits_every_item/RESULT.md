# 15 - One mouse move visits every item in the tree

**Status: quantified, root cause identified, core fix is candidate 12's.**

No scene here had ever dispatched an input event. Adding one `PointerMoved` per frame
(`--pointer-move`) to the existing scenes:

## Measured

`--mode scroll --count 100000`, 10 frames, medians of 3 runs:

| row style | items in tree | no input | +1 mouse move/frame | cost of the move |
| --- | --- | --- | --- | --- |
| `layout` (5 items/row) | ~500 000 | 126.99 ms | 165.28 ms | **+38.3 ms** |
| `clipped` (`clip: true`) | ~600 000 | 89.33 ms | 107.75 ms | **+18.4 ms** |
| `bare` (1 item/row) | ~100 000 | 20.71 ms | 30.48 ms | **+9.8 ms** |
| `ListView` (virtualized) | ~visible only | 2.26 ms | 2.27 ms | **+0.01 ms** |

`bare` gives the floor directly: 100 000 items, +9.8 ms, i.e. **~98 ns per item visited** for a
single mouse move. Scaling with model size, `layout` rows:

| rows | 20 000 | 40 000 | 70 000 | 100 000 |
| --- | --- | --- | --- | --- |
| cost of one move | +6.7 ms | +11.4 ms | +21.5 ms | +30.1 ms |

Linear in the **total** item count, not in the visible one.

## Root cause

`send_mouse_event_to_item` (`internal/core/input.rs:1663`):

```rust
let filter_result = if mouse_event.position().is_some_and(|p| geom.contains(p))
    || item.as_ref().clips_children()
{
    item.as_ref().input_event_filter_before_children(...)
} else {
    InputEventFilterResult::ForwardAndIgnore
};
```

When the cursor is **outside** an item's geometry the result is `ForwardAndIgnore`, and that maps to
`(forward_to_children: true, ignore: true)` - the item itself doesn't handle the event, but the
traversal still descends into **all of its children**. A row 3 000 000 px below the cursor has its
five children visited anyway.

It has to, as the code stands: Slint children are not clipped to their parent by default, so a child
may be somewhere its parent's geometry doesn't cover. Deciding otherwise needs to know the bounds of
the whole subtree - which is exactly the thing candidate 12 established the renderer also lacks.

`clip: true` halves the cost (+38.3 -> +18.4 ms) because `clips_children()` lets the filter run and
the clip stops the descent; the row item itself is still visited, so 100 000 visits remain.

## Why this is candidate 12's fix, on a third pass

Three separate traversals now measured with no notion of what is on screen:

| pass | candidate | cost at 100 000 rows |
| --- | --- | --- |
| rendering | 12 | ~5x more than the floor |
| layout (on data change) | 13 | 3 text shapes per changed row, visible or not |
| **input** | **15** | **~98 ns per item, per event** |

A cached subtree bounding box next to the existing per-item rendering cache would answer all three.
Candidate 12's constraint applies here too: it has to cost nothing per item, or the fix pays for
itself.

## Consequences

A UI with a non-virtualized list of 100 000 rows spends **~38 ms per mouse-move event** deciding
what is under the cursor - on top of the ~127 ms it already spends rendering. Mouse moves arrive at
the input device's rate, so hovering over such a list is as expensive as rendering it. Nothing about
this is visible in a benchmark that only scrolls.

## What to do about it today

1. **`ListView`** - immune, +0.01 ms.
2. **`clip: true`** on rows halves it, at the cost of the doubled allocations candidate 12 measured.
3. Fewer items per row helps proportionally (it is a per-item cost).

## Reproduce

```sh
tools/matrix.sh \
  "no input|--mode scroll --count 100000" \
  "+pointer|--mode scroll --count 100000 --pointer-move" \
  "bare no input|--mode scroll --count 100000 --row-style bare" \
  "bare +pointer|--mode scroll --count 100000 --row-style bare --pointer-move"
```
