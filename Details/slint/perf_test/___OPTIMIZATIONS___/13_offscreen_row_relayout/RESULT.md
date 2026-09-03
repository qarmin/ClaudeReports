# 13 - Changing an off-screen row re-shapes its text (non-virtualized lists)

**Status: quantified, application-level workaround measured, core fix not attempted.**

Candidates 01 and 12 measured what a non-virtualized `ScrollView` costs to *render*. This is the
same structure under *live data*, which nothing had measured: what does changing a row's contents
cost when the row is nowhere near the viewport?

## Measured

`--mode scroll --count 10000`, 10 frames, N rows rewritten with `set_row_data` per frame, all of
them far past the viewport (`--mutate-mode changed-offscreen`). Median of 3 runs.

| rows changed/frame | frame time | paragraph shapes/frame | allocations/frame |
| --- | --- | --- | --- |
| 0 | 10.49 ms | 0 | 30 817 |
| 20 | 11.42 ms | 60 | 32 252 |
| 50 | 12.17 ms | 150 | 34 401 |
| 100 | 13.20 ms | 300 | 37 985 |
| 200 | 17.33 ms | **600** | 45 150 |

**Exactly 3 text shapes per changed row** - one per `Text` in the row - and ~34 us of frame time
each. None of it is drawn: these rows are thousands of pixels outside the viewport.

The same run with rows whose children are positioned explicitly instead of by a layout
(`--row-style flat`):

| rows changed/frame | frame time | paragraph shapes/frame |
| --- | --- | --- |
| 0 | 7.35 ms | 18 |
| 100 | 7.33 ms | **18** |
| 200 | 7.29 ms | **18** |

**Zero.** The 18 are the rows scrolling into view, which is the work that has to happen. Changing
200 off-screen rows costs nothing at all.

Whether the changed rows are on-screen or off-screen makes no difference to the cost
(`changed` and `changed-offscreen` both report 60 shapes for 20 rows), which is the point: the
layout pass has no notion of visibility.

## Why

A `Text` only knows its size after its text is shaped. Putting the row's children in a
`HorizontalLayout` means the layout pass asks each child for its `layout_info`, so each `Text` in
every changed row is shaped - the layout pass runs over the whole model, because a plain `for`
inside a `ScrollView` instantiates every row. Positioning the children explicitly means nothing
asks for their `layout_info`, so the shaping only happens when a row is actually painted.

This is a *layout*-pass cost, not a rendering one, so it is not the same problem as candidate 12
(pruning culled subtrees while rendering) and would not be fixed by it.

## Contrast: `ListView` is not affected

`--mode list --count 100000`, same mutation:

| mutation | frame time | allocations/frame | shapes/frame |
| --- | --- | --- | --- |
| none | 2.20 ms | 1 502 | 18 |
| `set_row_data` x20, visible | 2.16 ms | 1 677 | 18 |
| `set_row_data` x20, off-screen | 2.20 ms | 1 677 | 18 |
| `push` x20 | 2.20 ms | 1 895 | 19 |
| `insert(0)` x20 | 2.95 ms | 2 962 | 43 |
| `remove(0)` + `push` x20 | 3.28 ms | 3 946 | 67 |

Virtualization removes the problem entirely: an off-screen row has no component, so changing it
costs a model notification and nothing else. The cost is also **flat in model size** - 1 000,
10 000 and 100 000 rows all behave the same - so `VecModel::insert`'s memmove is not a factor at
these sizes.

`insert(0)`/`rotate` cost more only because every visible row's *content* shifts by one, so the
visible rows genuinely have to be re-shaped (43 and 67 against a floor of 18, for ~25 visible
rows). That is correct behaviour, not overhead.

## Consequences

A dashboard-style UI (non-virtualized list, rows updating from live data) pays ~34 us per updated
row per frame regardless of what is on screen. At 200 rows/frame that is **6.8 ms of a frame spent
shaping text nobody sees**.

## What to do about it

1. **Use `ListView`.** It removes this completely and is the reason it exists.
2. If a plain `for` is unavoidable, **position row children explicitly rather than with a layout**.
   Measured on the same scene: 600 shapes/frame -> 18, frame 17.3 -> 7.3 ms, and separately the
   first frame 308 -> 15 ms and RSS 158 -> 46 MB at 10 000 rows.
3. `clip: true` on a layout row also gets the shapes down to 18, but **doubles allocations per
   frame** (61 417 against 32 252) - the same trap candidate 12 found.

## Possible core fix (not attempted)

`Text::layout_info` needs the shaped text for both orientations: the width is the text's width and
the height is its line count. For a single-line, non-wrapping `Text` the *height* is derivable from
font metrics alone, but the *width* is not, and layouts ask for both - so avoiding the shape needs
either the width to be known to be unused (the two fixed-width `Text`s in this row are a case where
it is) or a layout pass that knows the row is not visible. The second is the same missing notion of
visibility that candidate 12 needs, one pass earlier.

## No patch

This candidate has no `patch.diff`: the finding is a property of the framework's layout pass and the
fix measured here is a `.slint` structure change, which the harness already carries as
`--row-style flat` (`perf_test/ui/shared.slint`, `FlatRecordRow`). The core fix is described above
and was not attempted.

## Reproduce

```sh
tools/matrix.sh \
  "  0 changed|--mode scroll --count 10000" \
  "200 changed|--mode scroll --count 10000 --mutate 200 --mutate-mode changed-offscreen" \
  "200 flat   |--mode scroll --count 10000 --mutate 200 --mutate-mode changed-offscreen --row-style flat"
```
