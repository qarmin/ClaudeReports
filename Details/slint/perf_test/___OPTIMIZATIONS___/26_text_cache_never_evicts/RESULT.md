# 26 - The shaped-text cache never evicts: 4.68 KB per `Text` item, forever

Candidate 03 fixed one `TextInput` holding a whole document. This is the other shape of the same
problem: **many small `Text` items**, each holding its own shaped layout, with nothing ever
releasing them.

## Measured

`--mode scroll`, layout rows (3 `Text`s each) against `--row-style bare` (none), resident memory
after 10 frames:

| rows | `Text` items | RSS, layout rows | RSS, bare rows | retained per `Text` |
| --- | --- | --- | --- | --- |
| 2 000 | 6 000 | 44.7 MB | 17.3 MB | **4.68 KB** |
| 5 000 | 15 000 | 89.7 MB | 21.1 MB | **4.68 KB** |
| 10 000 | 30 000 | 164.4 MB | 27.2 MB | **4.68 KB** |
| 20 000 | 60 000 | 313.4 MB | 39.2 MB | **4.68 KB** |

Exactly linear, to two decimal places, across a 10x range. 60 000 short `Text` items retain
**274 MB** of shaped glyphs while ~25 of them are on screen.

Rows whose children are positioned explicitly (`--row-style flat`) retain **0.63 KB** per `Text`
instead: nothing asks for their layout-info, so nothing is shaped (candidate 13). The 4.68 KB is
the `parley::Layout` for a ~30 character string.

## Cause

`TextLayoutCache` is an `ItemCache<CachedParagraphs>` - a `HashMap` per component of a `HashMap`
per item index. Searching `internal/core/item_rendering.rs` for `evict`, `LRU` or `capacity`
returns **nothing**. An entry is released only when:

- the component is destroyed (`component_destroyed`),
- the window scale factor changes (`clear_cache_if_scale_factor_changed`),
- something calls `clear_all`,
- or the entry is invalidated by a property change - which *re-shapes* it rather than freeing it.

Nothing releases an entry because it has not been drawn for a while. A `Text` item that was laid
out once at startup keeps its glyphs for the lifetime of the component.

## Why candidate 03 does not help here

That candidate's lazy shaping is gated on documents of more than 128 paragraphs
(`RETAINED_PARAGRAPHS`), because below that the glyphs are cheap to keep and keeping them avoids
ever re-shaping. A one-line `Text` is one paragraph, so it takes the eager path by design. The
gate is right for a single item and wrong in aggregate: 60 000 items each individually "cheap to
keep" is 274 MB.

## Fix direction

The retention machinery candidate 03 built is per-*paragraph* inside one entry. What is missing is
the same idea one level up: a bound on the *cache*, evicting whole entries for items that have not
been drawn recently. The draw path already knows which items it visited, and
`CachedParagraphs::paragraphs` is already an `Option` that the code handles being `None` - an
evicted entry re-shapes on next use exactly like an invalidated one does today, so the correctness
surface is one that already exists.

The number to pick the bound with: 4.68 KB per entry, and ~25-50 entries are on screen in a list.

## Reproduce

```sh
for c in 2000 5000 10000 20000; do
  tools/matrix.sh \
    "$c layout|--mode scroll --count $c" \
    "$c bare|--mode scroll --count $c --row-style bare"
done
```
