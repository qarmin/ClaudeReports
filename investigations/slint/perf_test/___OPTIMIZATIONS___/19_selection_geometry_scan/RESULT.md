# 19 - Resolving a selection walks the whole document

**Status: fixed, measured, pixel-identical.** Patch: `selection_band_binary_search.diff`.

A selection makes the draw path resolve selection geometry on every frame. `--select-all` selects
the whole document once (Ctrl+A after focusing the input) and leaves it selected, which is what a
user does before copying.

## Measured

`--mode text --select-all`, medians of 3 runs, cost of having the selection:

| document | before | after |
| --- | --- | --- |
| 50 000 lines | +0.40 ms | - |
| 100 000 lines | +0.49 ms | **+0.31 ms** |
| 200 000 lines | +0.67 ms | - |
| 400 000 lines | **+1.09 ms** | **+0.43 ms** |

Before, the cost grew with the document: ~2 ns per paragraph, all of it in paragraphs that were
skipped. At 400 000 paragraphs that is ~0.8 ms per frame spent deciding not to do anything.

After, it is flat within noise - what remains is the fixed cost of resolving the spans that are
actually on screen.

## Cause and fix

`Layout::draw` already binary-searches the first paragraph that can reach the visible band:

```rust
let first = self.paragraphs.partition_point(|p| ... < visible_band.start).saturating_sub(1);
```

`selection_geometry` did not - it started at paragraph 0 and `continue`d past everything above the
band, and `continue`d again past everything below it rather than stopping. The fix is to mirror the
draw loop: binary-search the start, `break` at the end.

Both keep the same one-paragraph slack at each edge (glyph ink can overhang its line box, so the
edge paragraphs still get their spans and the per-paragraph test trims them).

## Validation

- **Pixels identical** on five scenes against a binary with the linear scan restored, including
  `--word-wrap` and a 400 000-line document scrolled 6 frames deep. The two binaries differ in
  time (5.22 ms vs 4.44 ms at 400 000 lines), so they are genuinely different builds.
- 16 `sharedparley` unit tests pass, including `bidi_selection_spans_are_ascending_in_x`, which is
  the one that would catch a mis-ordered or truncated span list.

## Reproduce

```sh
tools/matrix.sh \
  "400k no selection|--mode text --count 400000" \
  "400k select all|--mode text --count 400000 --select-all"
```
