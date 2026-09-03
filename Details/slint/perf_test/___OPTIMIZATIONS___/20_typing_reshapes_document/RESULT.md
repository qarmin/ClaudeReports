# 20 - Typing one character re-shapes the whole document

**Status: quantified, pre-existing (not a candidate 03 regression), fix designed but not attempted.**
The most user-visible number in this whole workspace.

## Measured

`--type-text` dispatches one character per frame into the focused `TextInput`, which is what typing
is. `--mode text --scroll-step 0`, medians of the frames after the first:

| document | per keystroke | paragraphs shaped | allocations |
| --- | --- | --- | --- |
| 1 000 lines | 22.85 ms | 1 050 | 16 842 |
| 10 000 lines | 180.41 ms | 10 050 | 160 846 |
| 50 000 lines | **882.35 ms** | 50 050 | 800 849 |

**One shape per paragraph per keystroke**, perfectly linear. Typing a single character into a
50 000-line document costs **0.9 seconds**; extrapolating, ~1.8 s at 100 000 lines. A text editor
built on `TextEdit` is unusable past a few thousand lines - not slow, unusable.

## Not a regression

| document | before candidate 03 | after |
| --- | --- | --- |
| 10 000 lines | 197.11 ms | 177.85 ms |
| 50 000 lines | 970.48 ms | 882.03 ms |

Candidate 03 made it marginally *better*. The behaviour is pre-existing and independent of the lazy
shaping work.

## Cause

The shaped-text cache is keyed per item and invalidated by the property dependencies its shaping
registered - the text among them. Change one character and the whole entry is dropped, so the next
layout re-splits the document into paragraphs and re-shapes every one of them, even though all but
one are byte-identical to what was just thrown away.

## Why candidate 03 makes the fix tractable

Before, a cache entry was an inseparable `Vec<parley::Layout>`: nothing in it could be salvaged
because nothing in it was addressable. After, a paragraph is `{ range, y, metrics, glyphs }` where
`metrics` is `{ width, height, line_count }` and the glyphs are already optional and reproducible
from the range. That is exactly the shape an incremental update needs:

1. Split the new text into paragraph ranges (a memchr scan, ~1 ms for 11 MB - already done today).
2. Find the first and last paragraph whose *content* differs from the previous split. A single
   keystroke changes exactly one, and everything after it only shifts in `range.start`.
3. Keep the metrics of every unchanged paragraph. Shift `range` by the byte delta and `y` by the
   height delta - both O(paragraphs) in *arithmetic*, no shaping.
4. Re-shape only the changed paragraph(s).

Step 3 is still O(paragraphs) per keystroke, but at ~2 ns each (the figure candidate 19 measured
for exactly this kind of scan) that is 0.2 ms for 100 000 paragraphs against the current 1 800 ms.

The obstacle is not the data structure any more, it is that the cache entry is destroyed by the
dependency tracker before anything can be salvaged from it. An incremental path needs the previous
entry to survive invalidation long enough to be diffed against - a "reshape with the old value in
hand" hook rather than the current drop-and-rebuild.

## Consequences

This is the ceiling on what candidate 03 achieved. Loading an 11 MB document is now cheap in memory
(38 MB instead of 540 MB) and reasonable in time (1.7 s), but *editing* it is not, and editing is
the point of a text editor. Anyone reaching for `TextEdit` for large documents needs to know this
before the memory numbers make it look solved.

## Reproduce

```sh
for c in 1000 10000 50000; do
  ../../target/rdebug/perf_test --bench --mode text --count $c --type-text --scroll-step 0 --frames 5
done
```
