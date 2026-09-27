# 20 - Typing one character re-shapes the whole document

Ported from `perf_test/___OPTIMIZATIONS___/20_typing_reshapes_document/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

The most user-visible number in this whole workspace: quantified and pre-existing (not a
candidate-003 regression), with a fix designed but not attempted.

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

| document | before candidate 003 | after |
| --- | --- | --- |
| 10 000 lines | 197.11 ms | 177.85 ms |
| 50 000 lines | 970.48 ms | 882.03 ms |

Candidate 003 made it marginally *better*. The behaviour is pre-existing and independent of the lazy
shaping work.

## Cause

The shaped-text cache is keyed per item and invalidated by the property dependencies its shaping
registered - the text among them. Change one character and the whole entry is dropped, so the next
layout re-splits the document into paragraphs and re-shapes every one of them, even though all but
one are byte-identical to what was just thrown away.

## Why candidate 003 makes the fix tractable

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

Step 3 is still O(paragraphs) per keystroke, but at ~2 ns each (the figure candidate 019 measured
for exactly this kind of scan) that is 0.2 ms for 100 000 paragraphs against the current 1 800 ms.

The obstacle is not the data structure any more, it is that the cache entry is destroyed by the
dependency tracker before anything can be salvaged from it. An incremental path needs the previous
entry to survive invalidation long enough to be diffed against - a "reshape with the old value in
hand" hook rather than the current drop-and-rebuild.

## Consequences

This is the ceiling on what candidate 003 achieved. Loading an 11 MB document is now cheap in memory
(38 MB instead of 540 MB) and reasonable in time (1.7 s), but *editing* it is not, and editing is
the point of a text editor. Anyone reaching for `TextEdit` for large documents needs to know this
before the memory numbers make it look solved.

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session.

## Why the result looks the way it does

See "Cause" and "Why candidate 003 makes the fix tractable" above: the shaped-text cache drops the
entire entry on any dependency change, so a one-character edit forces a full document re-shape
rather than an incremental one, and this is a pre-existing property of the cache design, not a
regression from prior work.

## What's not done / open questions

- The incremental reshape path described above (steps 1-4) was designed but never implemented or
  attempted in this session.
- Needs a "reshape with the old cache value in hand" hook in the dependency-invalidation path
  before an incremental diff-and-patch approach is possible at all.

## Reproduce

```sh
for c in 1000 10000 50000; do
  ../../target/rdebug/perf_test --bench --mode text --count $c --type-text --scroll-step 0 --frames 5
done
```
