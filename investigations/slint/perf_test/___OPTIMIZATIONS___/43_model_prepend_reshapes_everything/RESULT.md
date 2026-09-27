# 43 - one insertion at the front of a model re-shapes every row in the tree

Candidate 22 measured prepending as "**+8.73 ms** for 200 prepends into 200 000 rows, linear in
model size". That was measured on `bare` rows - rectangles with no text. With text in the row, the
number is **seventy times larger and has a different shape entirely**.

## It does not depend on how many rows you insert

`scroll` (non-virtualized), rows with three `Text` items each:

| prepends per frame | rows | frame | paragraph shapes |
| --- | --- | --- | --- |
| **1** | 20 000 | **617.27 ms** | 60 068 |
| 5 | 20 000 | 614.25 ms | 60 116 |
| 20 | 20 000 | 614.36 ms | 60 296 |
| 200 | 20 000 | 649.29 ms | 62 461 |

One insertion costs the same as two hundred. The cost is not per insertion.

## It is exactly linear in the number of rows

| rows | frame | paragraph shapes |
| --- | --- | --- |
| 5 000 | 150.67 ms | 15 068 |
| 20 000 | 617.27 ms | 60 068 |
| 40 000 | **1 250 ms** | 120 068 |

`3 x rows + 68` shapes, every time - three `Text` items per row, **every row in the tree**, not just
the visible ones. A single prepend into a 40 000-row list costs **1.25 seconds**.

## Why

Prepending shifts every row's data by one position, so every row's `record` property genuinely
changes and every `Text` in the tree re-shapes. That is candidate 13's finding (an off-screen row
re-shapes when its text changes) meeting candidate 22's, and the product is much worse than either.

Confirmed by removing the text: `scroll --row-style bare` with the same 200 prepends costs
**7.41 ms and 1 shape** - which is the +8.73 ms candidate 22 reported. That candidate measured the
model plumbing correctly and simply never had text in the row.

## Virtualization removes it entirely

| scene, 20 000 rows, 200 prepends | frame | shapes |
| --- | --- | --- |
| `scroll` (a `for` in a `ScrollView`) | 649.29 ms | 62 461 |
| `list` (`ListView`) | **4.65 ms** | **67** |

**139x.** `ListView` only instantiates visible rows, so only they re-shape. This is the strongest
argument for `ListView` measured anywhere in this workspace - stronger than the memory argument
candidate 01 made.

## The same signature appears elsewhere: a scale-factor change

`--scale-toggle` alternates the window scale factor every frame, which clears the shaped-text cache
wholesale (candidate 31):

| scene | frame | shapes |
| --- | --- | --- |
| `scroll` 20 000 | 21.51 ms | 18 |
| `scroll` 20 000 `--scale-toggle` | **603.48 ms** | **60 084** |

Identical arithmetic: `3 x rows`. So there are (at least) two triggers that re-shape every text in a
non-virtualized tree - **the model changing at the front, and the scale factor changing** - and both
cost about 30 us per row. Dragging a window between two monitors of different DPI does the second
one.

## What would fix it

Nothing at the model layer: the data really did change for every row. The fix is either
virtualization (already available, and 139x) or the same subtree bounds that candidates 12/13/15/32
want - a row that is not on screen should not shape its text until it is.

## Reproduce

```sh
RUNS=3 FRAMES=6 tools/matrix.sh \
  "1 prepend|--mode scroll --count 20000 --mutate 1   --mutate-mode insert-front" \
  "200 prepends|--mode scroll --count 20000 --mutate 200 --mutate-mode insert-front" \
  "ListView|--mode list --count 20000 --mutate 200 --mutate-mode insert-front"
```
