# 18 - What a level of nesting costs

Real dialogs nest ten or twenty components deep; every scene here was two or three deep, so the
cost of a level had never been measured.

New `nest` scene: 500 leaf rectangles, each wrapped in N levels, all visible. Slint components
cannot recurse, so the file is generated with each depth written out literally. Two wrappers:
`layouts` (a `VerticalLayout` per level, which asks its child for layout-info) and `plain`
(a `Rectangle` per level with explicit geometry).

| depth | total items | layouts | plain | allocations/frame | first frame |
| --- | --- | --- | --- | --- | --- |
| 0 | 500 | 0.96 ms | 0.96 ms | 11 | 1.78 ms |
| 2 | 1 500 | 1.01 ms | 1.08 ms | 11 | 1.83 ms |
| 4 | 2 500 | 1.16 ms | 1.08 ms | 12 | 2.03 ms |
| 8 | 4 500 | 1.21 ms | 1.23 ms | 12 | 2.12 ms |
| 16 | 8 500 | **1.65 ms** | **1.62 ms** | 13 | 4.10 ms |

## Two findings

**A level costs what an item costs, and nothing more.** 17x the items for 1.7x the frame time:
+0.69 ms for +8 000 items is **~86 ns per item per frame**. The wrappers are transparent and draw
nothing, so this is the pure cost of *visiting* an item.

That number keeps turning up: ~98 ns/item for hit-testing (candidate 15), ~86 ns/item here. **The
per-item visit is the engine's unit of currency**, and it is what candidates 12, 13 and 15 all pay
O(total items) of.

**A layout level is not more expensive than a plain one** - 1.65 ms against 1.62 ms at depth 16.
Layout results are cached and only recomputed when something is dirty, so a settled tree pays
nothing for its layouts per frame. Nesting is not what makes a UI slow; the *number of items* is.

## What the ~86 ns is made of

Built with `--features dep-stats` (candidate 08's registration counters; it distorts timings, so
only the counts are read):

| depth | items | dependency registrations/frame |
| --- | --- | --- |
| 0 | 500 | 4 183 (8.4 per leaf) |
| 16 | 8 500 | 12 183 |

The 8 000 extra wrapper items added **exactly 8 000 registrations - one per item**. A transparent
item that draws nothing still has one property read tracked on every frame, and that read plus the
visit machinery is the ~86 ns.

So the per-item cost is not rendering (these items draw nothing) and not layout (settled, cached).
It is the traversal itself plus one tracked property read. Candidate 14 removed the *allocation*
behind that read; the read itself remains, and at 100 000+ items it is what the O(total items)
passes are actually spending.

Instantiation does scale: the first frame goes 1.78 -> 4.10 ms for 500 -> 8 500 items, ~270 ns per
item created.

## Reproduce

```sh
for d in 0 2 4 8 16; do
  tools/matrix.sh "depth $d|--mode nest --count 500 --nest-depth $d"
done
```
