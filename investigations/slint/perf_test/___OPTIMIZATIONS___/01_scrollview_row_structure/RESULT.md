# 01 - Row structure in a non-virtualized ScrollView

**Status: win (application level, no Slint change needed).**
Full write-up with the mechanism and the core-side options: `../../FINDINGS.md`.

> **Measured before candidates 08-10 were applied.** On the current tree the absolute numbers are
> better (the `scroll` baseline is now ~133 ms/frame with 300 742 allocations and 1 424 MB instead
> of 148 ms / 701 063 / 1 542 MB), but the comparison between the variants - which is what this
> candidate is about - is unchanged. `FINAL_NUMBERS.md` has the current end-to-end table.

## Problem

`ScrollView` + `for` over 100 000 records: first frame 3.10 s, 148 ms/frame afterwards
(software renderer), RSS 1542 MB. `ListView` with the same data and row: 2.8 ms/frame,
36 MB.

## Cause (verified)

`render_item_children` (`internal/core/item_rendering.rs:210-230`) descends into the
children of items it just culled: when `filter_item` says invisible, `render_result` stays
`ContinueRenderingChildren`. Reaching the `HorizontalLayout` inside a row evaluates its
geometry -> children `layout_info` -> `ComplexText::layout_info` -> `text_size` -> parley
shaping. Confirmed by the shape counter: frame 0 does **300 000 text shapes** (100 000 rows
x 3 texts), frames 1..n do **0**. The shaped paragraphs stay in the text layout cache, which
is the 1.2 GB difference.

## Variants measured (100k records, software renderer, `--bench`)

| variant | first frame | later frames | allocations/frame | RSS |
| --- | --- | --- | --- | --- |
| baseline (row with a `HorizontalLayout`) | 3.10 s | 148 ms | 701 063 | 1542 MB |
| `--flat-rows` (children positioned explicitly) | 131 ms | 120 ms | 501 330 | 318 MB |
| `--flat-rows` + constant child geometry | 96 ms | 90 ms | 301 355 | - |
| baseline + `clip: true` on the row | 103 ms | 97 ms | 801 665 | - |
| row with no children at all (floor) | 35 ms | 29 ms | 300 459 | - |
| `ListView`, same row | 3.4 ms | 2.8 ms | 2 131 | 36 MB |

FemtoVG (Xvfb/llvmpipe) confirms the same ordering: baseline 2.73 s / 55.6 ms,
`--flat-rows` 171 ms / 46.4 ms, flat+constant 111 ms / 36.2 ms, `ListView` 109 ms / 5.3 ms.

```sh
../target/rdebug/perf_test --bench --mode scroll --count 100000 --frames 6 [--flat-rows]
just frames-xvfb 100000 8
```

## Patches

- `flat_rows.diff` - the shipped `FlatRecordRow` variant (runtime switch `--flat-rows` /
  checkbox), i.e. a row whose children have explicit geometry instead of a layout.
- `clip_rows.diff` - one line, `clip: true` on the row background. Works because
  `visit_clip` (`item_rendering.rs:519-539`) returns `ContinueRenderingWithoutChildren` for
  an empty clip region - the only subtree pruning the renderer has today. Not shipped in the
  app so that the default reproduces the original profile.

## What this means for Slint itself

See `../../FINDINGS.md` section 3: pruning subtrees of culled items (needs a cached subtree
bounding box, or a compiler flag for components whose children provably stay inside the
root) is the only change that removes the pathological first frame *and* the retained
shaping cache for everyone. A `DependencyNode` arena (#11292/#12255) is worth ~1.6-1.8x on
FemtoVG here (measured in `../../FINDINGS.md` section 2) but does not change the
complexity.
