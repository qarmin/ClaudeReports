# 19 - Resolving a selection walks the whole document

Ported from `perf_test/___OPTIMIZATIONS___/19_selection_geometry_scan/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

A selection makes the draw path resolve selection geometry on every frame. `--select-all` selects
the whole document once (Ctrl+A after focusing the input) and leaves it selected, which is what a
user does before copying. Patch: `patch.diff`.

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

## Does it change behavior/output?

- **Bit-exact?** yes - pixels identical on five scenes against a binary with the linear scan
  restored, including `--word-wrap` and a 400 000-line document scrolled 6 frames deep. The two
  binaries differ in time (5.22 ms vs 4.44 ms at 400 000 lines), so they are genuinely different
  builds, not a no-op diff.
- **Is this a breaking change?** no.
- Project test suite: 16 `sharedparley` unit tests pass, including
  `bidi_selection_spans_are_ascending_in_x`, which is the one that would catch a mis-ordered or
  truncated span list. No new failures.

## Why the result looks the way it does

The draw loop already had the binary-search-plus-break shape; `selection_geometry` was written
as a plain linear scan over the same paragraph list and never got the same treatment, so it paid
an O(document) cost every frame a selection existed regardless of how much of the document was
actually visible.

## What's not done / open questions

- None noted in the original session - patch is measured and validated as above.

## Reproduce

```sh
tools/matrix.sh \
  "400k no selection|--mode text --count 400000" \
  "400k select all|--mode text --count 400000 --select-all"
```

## Reproducibility check after upstream update (2026-08-23)

`patch.diff` still applies cleanly against slint HEAD `cdb6c43f7`. Not rebuilt/re-benched this round - re-verified via `git apply --check` only.
