# 24 - Rows with different font sizes cost ~50% more per frame

Ported from `perf_test/___OPTIMIZATIONS___/24_font_size_variety/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

**Correction on file:** `CORRECTION.md` (copied alongside this file) revisits this candidate and
finds the original headline attribution wrong - the cost tracked font *size*, not the *number of
distinct sizes*, because the original scene's `--font-sizes N` knob conflated "more variety" with
"bigger text". See that file for the full re-measurement; the summary is folded into this RESULT.md
below.

Every scene here used one font size for everything. Real lists do not: a chat has headings, a log
has severity levels, a table has emphasised cells. Each distinct `(font, size)` pair is its own
glyph-rasterization cache entry, so a UI that varies size per row stresses a cache a uniform list
never touches.

`--font-sizes N` cycles the rows through N sizes (13px..13+N-1 px, kept small so row heights do not
change and the layout stays comparable).

## What was tried (original measurement)

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

## Result (original reading, since corrected below)

Varying font size per row is not free but it is bounded: the cost is proportional to the number of
*distinct sizes visible at once*, not to how many the document uses. Two or three sizes (the normal
case) measure as free. A design that gives every row its own size pays ~50%.

Untested at the time: the same question for font *families* and weights, which are separate cache
keys again and where fallback resolution is also involved.

## Correction (see `CORRECTION.md`)

The scene's `--font-sizes N` computed each row's size as
`13px + mod(variant, N) * 1px`, so `--font-sizes 128` did not just add 128 distinct sizes - it also
grew the *largest* size to 140px. "More variety" and "bigger text" were the same knob, and the
sweep could not separate them. After decoupling them with an independent `--font-size <px>` base
(baselines unchanged within spread: `list` 2.28 -> 2.33 ms, `list --font-sizes 128` 4.62 -> 4.57 ms,
`scroll` 21.62 -> 21.95 ms):

- **Size alone** (one distinct size, `list` 20 000 rows, RUNS=7): 13px 2.86 ms -> 48px 4.01 ms,
  **+40%**, then flat past 48px (100px: 3.94 ms) - the plateau is rows clipping glyph height past
  their own 34px height.
- **Variety alone** (fixed size, varying count): 1 distinct size at 76px = 3.98 ms; 2 distinct sizes
  at 76-77px = 4.02 ms (no measurable cost); 128 distinct sizes at 13-140px = 4.56 ms; 128 distinct
  sizes at 1-128px = 4.44 ms. Going from 1 to 128 distinct sizes at otherwise-comparable magnitude
  costs **~+15%**, not the +57% originally attributed to variety.
- **Glyph cache is not the mechanism**: raising `GLYPH_CACHE` (`software/fonts/vectorfont.rs:65-72`)
  from its 1 MB cap to 16 MB changed nothing (1 size: 2.26 vs 2.25 ms; 32 sizes: 3.76 vs 3.60 ms;
  128 sizes: 4.46 vs 4.41 ms) - this null result closes plan item A4.

**Corrected conclusion:** rasterization cost tracks roughly with glyph area (i.e. font size), up to
where the row clips it, not with the number of distinct sizes in play. A UI with large type costs
meaningfully more (+40% here); a UI with many different type sizes at comparable magnitude is barely
more expensive than one with a single size (+15% at the extreme of 128 sizes, ~0% at 2 sizes).

## Reproduce

```sh
for n in 1 8 32 128; do
  tools/matrix.sh "$n sizes|--mode list --count 100000 --font-sizes $n"
done
```

## Does it change behavior/output?

N/A - measured/quantified only, no patch applied to real source in the original session.

## Why the result looks the way it does

The original scene conflated two independent variables (size and variety-count) behind one CLI
flag, so the +57% it measured was mostly a font-size effect. Once decoupled, the picture is
straightforward: rasterization cost scales with glyph area up to the point the container clips it,
and paying for many distinct sizes at a comparable magnitude is nearly free because the glyph cache
handles cache-key growth without measurable overhead (confirmed by the 1 MB vs. 16 MB cache-size
test showing no effect).

## What's not done / open questions

- Font *families* and weights (separate cache keys, plus fallback resolution) were not tested here.
- No patch was proposed or applied - this candidate is a measurement/quantification exercise only.
