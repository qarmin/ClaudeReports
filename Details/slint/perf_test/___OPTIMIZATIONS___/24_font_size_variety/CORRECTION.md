# 24, corrected - it was the font *size*, not the number of sizes

Candidate 24 measured `--font-sizes 128` at **+57%** per frame and concluded:

> the cost is per *distinct size visible at once*, not per model. Glyph rasterization, not
> re-shaping (shapes stay 18/frame).

The second half is right. The first half is wrong, and the scene is why.

## The confound

`shared.slint` computed the row's font size as:

```slint
property <length> size: 13px + mod(root.variant, max(1, Settings.font-size-variants)) * 1px;
```

So `--font-sizes 128` does not only produce 128 distinct sizes - it produces sizes from **13px to
140px**. "More variety" and "bigger text" were the same knob, and the sweep could not tell them apart.

The scene now takes a `font-size-base` (`--font-size <px>`), defaulting to the 13px that was
hard-coded, so the two can be varied independently. Baselines are unchanged: `list` 2.28 -> 2.33 ms,
`list --font-sizes 128` 4.62 -> 4.57, `scroll` 21.62 -> 21.95, all within the spread.

## Size alone, with exactly one distinct size

`list` 20 000, `RUNS=7`:

| font size | frame |
| --- | --- |
| 13 px | 2.86 ms |
| 20 px | 2.70 ms |
| 26 px | 3.06 ms |
| 34 px (= the row height) | 3.63 ms |
| 48 px | 4.01 ms |
| 100 px | 3.94 ms |

**+40% from 13px to 48px with a single size throughout**, then flat. The plateau is the row: rows are
34px tall, so past roughly that the extra glyph height is clipped away and stops costing anything.
That also explains the plateau candidate 24 saw between 128 and 512 variants and did not account for.

## Variety alone

| config | frame |
| --- | --- |
| 1 distinct size, 76px | 3.98 ms |
| **2** distinct sizes, 76-77px | 4.02 ms |
| 128 distinct sizes, 13-140px | 4.56 ms |
| 128 distinct sizes, 1-128px | 4.44 ms |

Doubling the number of distinct sizes at the same size costs **nothing** (3.98 -> 4.02, inside the
spread). Going to 128 distinct sizes adds 0.58 ms over a single large size - about **+15%**, not the
+57% that was attributed to variety.

## And it is not the glyph cache

The obvious mechanism for a per-distinct-size cost is the glyph cache thrashing: `GLYPH_CACHE`
(`software/fonts/vectorfont.rs:65-72`) is a `clru` cache capped at **1 MB**. Raising it to 16 MB and
re-running:

| | 1 MB | 16 MB |
| --- | --- | --- |
| 1 size | 2.26 ms | 2.25 ms |
| 32 sizes | 3.76 ms | 3.60 ms |
| 128 sizes | 4.46 ms | 4.41 ms |

No effect. The cap is not the constraint, so plan item **A4 is a null result** - and the "cheapest
entry in the list" turned out to be cheap and worth nothing, which is the point of writing the null
condition down first.

## What candidate 24 should say

Rasterizing text costs roughly in proportion to the glyph area, up to the point where the container
clips it. A UI with large type is more expensive than one with small type by a margin that matters
(+40% of a whole frame here); a UI with *many different* type sizes is barely more expensive than one
with a single size. The practical advice inverts: worry about how big the text is, not about how many
sizes the design uses.
