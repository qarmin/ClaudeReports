# 15, re-checked after candidates 35 and 36

Candidate 15 measured **+38 ms per mouse move** at 100 000 rows, before the rendering traversal was
made cheaper. Since 35 and 36 cut that traversal by 12% and 8%, the question is whether the input
pass moved with it. It did not, and it should not have: they are different traversals.

`RUNS=7`, same binary, with and without `--pointer-move`:

| scene | no input | one mouse move per frame | cost |
| --- | --- | --- | --- |
| `scroll` 100 000 (layout rows) | 115.58 ms | 147.25 ms | **+31.67 ms** |
| `scroll` 100 000 `bare` | 17.02 ms | 26.49 ms | **+9.47 ms** |
| `list` 100 000 (`ListView`) | 2.23 ms | 2.25 ms | +0.02 ms |

`bare` is one item per row, so +9.47 ms over 100 000 items is **~95 ns per item** - candidate 15
measured ~98 ns, so the per-item cost is unchanged.

**What did change is its share.** With the render traversal cheaper, hit-testing is now **36% of the
frame** in the `bare` scene (9.47 of 26.49 ms). Making one traversal faster moves the bottleneck to
the next one, which is the argument for fixing them at the source rather than one at a time.

## An idea that did not work

`send_mouse_event_to_item` (`core/input.rs:1652`) calls `item_rc.geometry()` for every item, and it
is a **tracked** read - the same shape as `filter_item` before candidate 35, and candidate 33 found
that registration was 26% of the dirty-region walk. So: read it untracked and see.

| | tracked (baseline) | untracked |
| --- | --- | --- |
| `scroll` 100 000 `bare --pointer-move` | 26.52 ms | 27.03 ms |

**+1.9%, faster in 1 of 7 pairs** - untracked is if anything *slower*, because `evaluate_no_tracking`
has its own cost and there was no registration saving to pay for it. Unlike the dirty-region walk,
the input pass's per-item cost is the visit and the geometry arithmetic, not the bookkeeping.

Both halves of the geometry are genuinely needed here, too: the origin to translate the event into
child coordinates, the size for `geom.contains(p)`. Candidate 35's trick - return the origin and skip
the size - has nothing to skip.

So candidate 15 still points where it pointed: subtree bounds, or nothing.
