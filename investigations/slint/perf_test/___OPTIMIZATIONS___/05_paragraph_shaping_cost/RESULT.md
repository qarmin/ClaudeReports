# 05 - Where the ~20 us per paragraph of shaping actually goes

**Status: quantified. No contained win exists at the Slint level; the cost is parley's shaped
representation. Redirects to 03's "lazy/windowed shaping" item.**

Two hypotheses were tested and both are null; the third measurement is the useful one.

## Null 1: the per-paragraph fontique query

`LayoutWithoutLineBreaksBuilder::ranged_builder`
(`internal/core/textlayout/sharedparley/shaping.rs:74-84`) queries fontique for font metrics
to pin the line height - and it runs **per paragraph**, so a 100 000 line document does
100 000 font queries. Ablation: hard-code `line_height_ratio = None` (skipping the query
entirely) and measure the text mode.

```
baseline    show + layout  2.08 s  (1 500 203 allocations)  RSS 540.8 MB
no query    show + layout  2.08 s  (1 500 203 allocations)  RSS 541.2 MB
```

Unchanged - fontique's query is cached and cheap. Patch reverted, nothing to do.

## Null 2: batching paragraphs into fewer parley layouts

Slint builds one `parley::Layout` per paragraph. `parley_stage_bench/` measures the same
corpus with several paragraphs per layout (parley handles the mandatory breaks at `'\n'`
itself), with exact live-byte accounting (a tracking global allocator - RSS deltas are
useless across variants because the allocator reuses freed memory):

| paragraphs per layout | time per paragraph | layouts | live bytes retained |
| --- | --- | --- | --- |
| 1 (what Slint does) | 17.4 us | 100 000 | 484.5 MB |
| 10 | 15.7 us | 10 000 | 508.5 MB |
| 100 | 15.5 us | 1 000 | 398.4 MB |
| 100 000 (whole document) | 18.2 us | 1 | 1004.2 MB |

Chunking buys ~10% of time and nothing reliable in memory; one layout for the whole document
is the worst of both. So the per-paragraph split Slint chose is not the problem.

## The useful measurement: it is per character, and it is the retained glyph data

Same bench, same 10 MB of text redistributed into different line lengths:

| paragraphs x chars | build (analysis+shape) | per paragraph | per character | retained per paragraph |
| --- | --- | --- | --- | --- |
| 100 000 x 100 | 1.52 s | 15.2 us | 146 ns | 5.07 KB |
| 10 000 x 1 000 | 1.25 s | 124.9 us | 125 ns | 26.06 KB |
| 1 000 x 10 000 | 1.24 s | 1243.6 us | 124 ns | 240.11 KB |
| 100 000 x 20 | 0.48 s | 4.8 us | 208 ns | 2.82 KB |

- **~124 ns per character** to analyze + shape, plus a ~2.3 us per-paragraph constant (15% at
  100 chars/line). Of the per-character cost, ICU line+word segmentation is only ~10 ns
  (candidate 04).
- **~50 bytes per character retained** in the `parley::Layout` (plus ~2.5 KB fixed per
  layout): 5.07 KB for a 100-character line, which is exactly the 5.25 KB/line that
  candidate 03 measured through Slint.
- `break_all_lines` is 0.5 us per paragraph, i.e. 3% - not where the time is.

```sh
cd parley_stage_bench && cargo run --release -- 100000 100
cd segmenter_bench   && cargo run --release            # candidate 04
```

## Conclusion

For a 10 MB document, ~1.5 s of shaping and ~0.5 GB of glyph data are what parley costs at
124 ns and 50 bytes per character. Neither the per-paragraph setup nor the chunking nor the
segmenter is a meaningful lever. The only lever left is **not doing it for the whole
document**: shape for metrics and drop (O(1) memory), retain glyph layouts only for the
paragraphs actually drawn, and find a way to not need the exact widest line before the user
has scrolled there. That is the open item in `../03_text_document_shaping/RESULT.md`, and it
is a design change in `i-slint-core`'s text layout cache, not a local optimization.
