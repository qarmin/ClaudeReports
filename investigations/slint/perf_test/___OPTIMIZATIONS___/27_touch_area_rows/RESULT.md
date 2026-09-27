# 27 - What a `TouchArea` per row costs

Clickable list rows are everywhere and no scene had one. `--row-style touch` is the realistic row
plus a `TouchArea` whose `has-hover` drives a highlight (reading it is what makes the hover state
actually propagate instead of being optimized away).

## Measured

`--mode scroll --count 5000`, medians of 7 runs. **This scene is noisy** - the spreads are given
because they are large enough to matter:

| | frame time | spread | allocations | RSS |
| --- | --- | --- | --- | --- |
| layout rows | 5.02 ms | 1.53 | 170 | 90.1 MB |
| layout rows + 1 mouse move/frame | 5.49 ms | 0.59 | 367 | 89.7 MB |
| touch rows | 6.42 ms | 2.28 | 176 | 92.3 MB |
| touch rows + 1 mouse move/frame | 7.41 ms | 3.58 | 384 | 92.1 MB |

Two readings, both consistent with what is already known and neither surprising:

**Statically, +1.4 ms for 5 000 rows.** The touch row adds two items (the `TouchArea` and the
highlight `Rectangle`), so 10 000 extra items at the ~90 ns per-item visit measured in candidates
15 and 18 predicts ~0.9 ms. The measurement is 1.4 ms with a 1.5-2.3 ms spread, so it is consistent
with "a `TouchArea` costs what an item costs" and does not support anything stronger.

**The per-event hit-test cost roughly doubles**: +0.47 ms without the touch areas, +0.99 ms with
them. Each `TouchArea` is another item that `send_mouse_event_to_item` visits *and* runs
`input_event_filter_before_children` on, so it is more than a bare visit - which is exactly the
O(total items) traversal candidate 15 documented, now with a heavier per-item body.

RSS rises 2.2 MB for 10 000 extra items, ~0.22 KB each.

## Reading

There is **no special `TouchArea` penalty** to find here: it costs what items cost, and it makes
candidate 15's problem proportionally worse because it makes the per-item work larger. A clickable
non-virtualized list of 100 000 rows would pay candidate 15's ~38 ms per mouse move roughly twice
over.

The honest limit of this measurement is the noise: with spreads of 1.5-3.6 ms on a 5-7 ms frame,
these numbers support "same order as the per-item cost" and not a precise multiplier. A quieter
scene (the `rects` grid) would be needed to sharpen it, which would mean putting `TouchArea`s there.

## Reproduce

```sh
RUNS=7 tools/matrix.sh \
  "layout|--mode scroll --count 5000" \
  "touch|--mode scroll --count 5000 --row-style touch" \
  "touch +pointer|--mode scroll --count 5000 --row-style touch --pointer-move"
```
