# 15 - One mouse move visits every item in the tree

Ported from `perf_test/___OPTIMIZATIONS___/15_hit_test_visits_every_item/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Quantified, root cause identified; the core fix is candidate 012's (a cached subtree bounding box).

No scene here had ever dispatched an input event. Adding one `PointerMoved` per frame
(`--pointer-move`) to the existing scenes:

Two follow-up notes from later sessions are copied verbatim alongside this file and summarized
below: `RECHECK_AFTER_35_36.md` re-measured the cost after candidates 035/036 made the rendering
traversal cheaper (headline: the input-pass per-item cost is unchanged at ~95-98 ns/item, but its
*share* of the frame grew to 36% in the `bare` scene since the other traversal got faster - and a
"read the geometry untracked" idea was tried and made things very slightly worse, not better).
`CLIP_CORRECTION.md` corrects this file's original claim that `clip: true` halves the hit-test cost
(headline: re-measured it is actually **-18%, not -50%**, and the residual cost is dominated by a
~40 ns-per-item geometry read rather than by the number of items visited, including a discovered
duplicate geometry read - once by `send_mouse_event_to_item`, once by
`Clip::input_event_filter_before_children` - worth ~4.5 ms/move at 100 000 clipped rows but not
fixed because of a one-pixel edge-inclusion mismatch between the two checks).

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

(Note: `CLIP_CORRECTION.md` later re-measured the `clipped` row at 100 000 rows and found the move
costs +29.49 ms there against a +35.97 ms `layout` baseline on the current tree - a **-18%**
reduction, not the -50% figure implied above; see that file for the full re-measurement.)

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
the whole subtree - which is exactly the thing candidate 012 established the renderer also lacks.

`clip: true` was originally measured to halve the cost (+38.3 -> +18.4 ms) because `clips_children()`
lets the filter run and the clip stops the descent, with the row item itself still visited (100 000
visits remaining). `CLIP_CORRECTION.md` (see summary above and the file itself) found this halving
does not hold up under re-measurement on the current tree - the real reduction is -18%, and the
residual cost is dominated by a per-item geometry read rather than the number of items visited.

## Why this is candidate 012's fix, on a third pass

Three separate traversals now measured with no notion of what is on screen:

| pass | candidate | cost at 100 000 rows |
| --- | --- | --- |
| rendering | 12 | ~5x more than the floor |
| layout (on data change) | 13 | 3 text shapes per changed row, visible or not |
| **input** | **15** | **~98 ns per item, per event** |

A cached subtree bounding box next to the existing per-item rendering cache would answer all three.
Candidate 012's constraint applies here too: it has to cost nothing per item, or the fix pays for
itself.

## Consequences

A UI with a non-virtualized list of 100 000 rows spends **~38 ms per mouse-move event** deciding
what is under the cursor - on top of the ~127 ms it already spends rendering. Mouse moves arrive at
the input device's rate, so hovering over such a list is as expensive as rendering it. Nothing about
this is visible in a benchmark that only scrolls.

## What to do about it today

1. **`ListView`** - immune, +0.01 ms.
2. **`clip: true`** on rows reduces it (measured -18% on re-check, see `CLIP_CORRECTION.md`, not the
   originally reported -50%), at the cost of the doubled allocations candidate 012 measured.
3. Fewer items per row helps proportionally (it is a per-item cost).

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session (nor in the
two follow-up recheck sessions, which also only measured and ablated in a benchmark harness).

## Why the result looks the way it does

The input-event traversal has no notion of on-screen visibility, so it walks every item in the tree
for every pointer move regardless of where the pointer is; this is architecturally the same gap
candidates 012 and 013 found in the rendering and layout passes respectively, just hit by a third
traversal. See `RECHECK_AFTER_35_36.md` and `CLIP_CORRECTION.md` for why later attempts to shrink
it further (untracked geometry reads, `clip: true`) only partially work.

## What's not done / open questions

- The core fix (cached subtree bounding box) was not attempted here; it is shared work with
  candidate 012.
- `CLIP_CORRECTION.md` identifies a genuine duplicate geometry read (worth ~4.5 ms/move at 100 000
  clipped rows) that was deliberately **not** fixed because `LogicalRect::contains` and the `Clip`
  check disagree on edge inclusion by one pixel - a behavioral decision left for a maintainer.

## Reproduce

```sh
tools/matrix.sh \
  "no input|--mode scroll --count 100000" \
  "+pointer|--mode scroll --count 100000 --pointer-move" \
  "bare no input|--mode scroll --count 100000 --row-style bare" \
  "bare +pointer|--mode scroll --count 100000 --row-style bare --pointer-move"
```
