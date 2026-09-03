# 01 - Row structure in a non-virtualized ScrollView

Ported from `perf_test/___OPTIMIZATIONS___/01_scrollview_row_structure/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Application level, no Slint core change needed.

> **Note on the patches in this folder:** both diffs target `perf_test/ui/shared.slint`, and
> `perf_test/` is untracked in the main `slint` git repo - a fresh clone of `repo_url` will not
> contain it at all. See "What's not done / open questions" below for how to apply these two
> diffs yourself.

> **Measured before candidates 008-010 were applied.** On the current tree the absolute numbers are
> better (the `scroll` baseline is now ~133 ms/frame with 300 742 allocations and 1 424 MB instead
> of 148 ms / 701 063 / 1 542 MB), but the comparison between the variants - which is what this
> candidate is about - is unchanged.

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

## What was tried

Variants measured (100k records, software renderer, `--bench`):

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

- `patch.diff` - the shipped `FlatRecordRow` variant (runtime switch `--flat-rows` /
  checkbox), i.e. a row whose children have explicit geometry instead of a layout.
- `patch_alt_clip_rows.diff` - one line, `clip: true` on the row background. Works because
  `visit_clip` (`item_rendering.rs:519-539`) returns `ContinueRenderingWithoutChildren` for
  an empty clip region - the only subtree pruning the renderer has today. Not shipped in the
  app so that the default reproduces the original profile.

## Result

First frame goes from 3.10s to 0.13s and RSS from 1542MB to 318MB by restructuring the row's
`.slint` markup alone (`--flat-rows`), with no change to Slint core. Adding constant child
geometry on top brings it further down to 96ms/301 355 allocations. `clip: true` on the row is a
cheaper one-line alternative (103ms) but nearly doubles allocations per frame (701 063 -> 801 665)
since every row gains a `Clip` item.

## Does it change behavior/output?

N/A in the sense that no Slint core code was touched - both diffs are `.slint`-level changes to
the `perf_test` harness UI. Applying `patch.diff` changes `perf_test`'s own row layout
(explicit child geometry instead of a `HorizontalLayout`), which is a visible structural choice in
the demo app, not a framework behavior change.

## Why the result looks the way it does

`render_item_children` keeps descending into a culled item's children (looking for anything that
might still need rendering), so a row's `HorizontalLayout` gets its geometry evaluated - and
therefore its children's `layout_info`, and therefore text shaping - even when the row itself is
never drawn. Removing the layout (explicit child positions) or pruning the subtree (`clip: true`)
both avoid this; only the compiler/core knowing the subtree can't escape its parent's bounds
would fix it structurally for every app, not just this row.

## What's not done / open questions

- `perf_test/` is untracked in the main `slint` git repo, so a fresh `git clone` of the main repo
  will not contain `perf_test/ui/shared.slint` at all - `patch.diff` and `patch_alt_clip_rows.diff`
  cannot be applied via a self-contained clone-based justfile in this folder. To apply them
  yourself: copy the `.diff` file into your own local `perf_test/` checkout and apply it there
  directly, e.g.
  ```sh
  cd path/to/perf_test && patch -p1 < patch.diff
  # or, if that perf_test checkout is its own git repo:
  cd path/to/perf_test && git apply patch.diff
  ```
- Pruning subtrees of culled items (needs a cached subtree bounding box, or a compiler flag for
  components whose children provably stay inside the root) is the only change that would remove
  the pathological first frame *and* the retained shaping cache for every app, not just this one -
  see candidate `012.SUBTREE_PRUNING_PRIZE`. A `DependencyNode` arena (upstream #11292/#12255) is
  worth ~1.6-1.8x on FemtoVG here but does not change the underlying complexity.

## Reproducibility check after upstream update (2026-08-23)

Status unchanged: both diffs still target `perf_test/ui/shared.slint`, still untracked in the main repo, still not clone-checkable. Not re-verified beyond confirming this - see "What's not done / open questions" above for how to apply them manually if needed.
