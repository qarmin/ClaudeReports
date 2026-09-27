# 03 - TextInput shapes the whole document (plan, written before the experiments)

## Observation that started this

`--mode text`, one `TextInput` in a `ScrollView`, no wrap:

| lines | RSS after 2 frames |
| --- | --- |
| 1 000 | 19.9 MB |
| 10 000 | 67.2 MB |
| 100 000 | 540.8 MB |

10 000 -> 100 000 lines is +473 MB for +90 000 lines = **5.25 KB per line**, for lines of
~110 characters. The text itself is 11 MB; the resident cost is ~48x that. Meanwhile
per-frame cost is fine (3.8 ms, 0 shapes/frame) - so this is a one-off, retained cost.

The frame table shows 0 text shapes in frame 0 too, so the shaping happens before the first
frame - during the layout pass triggered by `show()` (`preferred-width`/`preferred-height`
of the `TextInput`, which my `TextArea` reads for `content-width`/`content-height`, exactly
like Slint's own `TextEdit`).

## Suspected mechanism (to verify)

`create_text_paragraphs` (`internal/core/textlayout/sharedparley/shaping.rs:265-300`) loops
over *every* paragraph of the text and builds a `parley::Layout` per paragraph:

```rust
for range in paragraph_ranges(text) {
    paragraphs.push(paragraph_from_text(font_context, &text[range], ...));
}
```

`TextParagraph` then retains that `parley::Layout<Brush>` (glyphs, runs, clusters, lines)
for every paragraph of the document, cached per item in `TextLayoutCache`. Nothing here is
lazy or windowed, so cost and memory are O(document), not O(visible).

## Experiments

| # | Experiment | Purpose |
| --- | --- | --- |
| T1 | `TextArea` with `content-width`/`content-height` fixed (never read `preferred-*`), no wrap | Is `preferred-*` the only trigger, or does drawing alone shape the whole document? |
| T2 | Time the fill+show phase separately for 10k/100k lines | Cost per line of document, not just memory |
| T3 | Read the shape counter before the first frame's reset | Confirm the shaping count equals the paragraph count |
| T4 | If T1 shows `preferred-*` is the trigger: what does `preferred-height` actually need? For `no-wrap` it is `paragraph_count * line_height`, which needs no shaping at all | Find the cheapest correct patch |

## Possible fixes to evaluate (in increasing size)

1. `preferred-height` for `wrap: no-wrap`: count paragraphs, multiply by line height from
   the font metrics, never shape. `preferred-width` still needs advances.
2. Lazy per-paragraph shaping: `TextParagraph.layout: Option<Layout>`, shaped on first use
   (draw / hit-test / measure of that paragraph), plus an eviction policy so a 10 MB
   document does not retain every paragraph forever.
3. Cheaper retained representation (advances only, re-shape on draw) - large, parley-side.

Risks to check for any of these: `preferred-width` semantics (scrollbar range), cursor
rect / hit-testing correctness, selection rendering, and the existing tests
`api/rs/slint/tests/text_*.rs` plus `tests/screenshots`.
