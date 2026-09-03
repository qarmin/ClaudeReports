# 24 - Rows with different font sizes cost ~50% more per frame

Every scene here used one font size for everything. Real lists do not: a chat has headings, a log
has severity levels, a table has emphasised cells. Each distinct `(font, size)` pair is its own
glyph-rasterization cache entry, so a UI that varies size per row stresses a cache a uniform list
never touches.

`--font-sizes N` cycles the rows through N sizes (13px..13+N-1 px, kept small so row heights do not
change and the layout stays comparable).

| distinct sizes | frame time | allocations/frame | RSS |
| --- | --- | --- | --- |
| 1 | 2.82 ms | 955 | 36.9 MB |
| 2 | 2.59 ms | 957 | 36.9 MB |
| 8 | 2.46 ms | 977 | 37.2 MB |
| 32 | 3.61 ms | 1 192 | 37.7 MB |
| 128 | **4.44 ms** | 1 264 | 38.3 MB |
| 256 | 4.48 ms | 1 264 | 38.4 MB |

**+57% at 128 sizes, and it plateaus there** - only ~25 rows are visible, so at most ~25 distinct
sizes are in play per frame however many the model contains. The plateau is the tell that this is a
per-visible-row cost, not a per-model one: nothing is being retained badly, the glyphs simply have
to be rasterized at each size.

Paragraph shapes stay at 18/frame throughout, so this is **not** re-shaping - it is glyph
rasterization. Allocations rise 955 -> 1 264, which is the cache filling.

## Practical reading

Varying font size per row is not free but it is bounded: the cost is proportional to the number of
*distinct sizes visible at once*, not to how many the document uses. Two or three sizes (the normal
case) measure as free. A design that gives every row its own size pays ~50%.

Untested here: the same question for font *families* and weights, which are separate cache keys
again and where fallback resolution is also involved.

## Reproduce

```sh
for n in 1 8 32 128; do
  tools/matrix.sh "$n sizes|--mode list --count 100000 --font-sizes $n"
done
```
