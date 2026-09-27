# 44 - `clip: true` on a row turns 60 081 startup shapes into 66

Candidate 01 found that a non-virtualized `ScrollView` starts far faster with `--flat-rows` **and**
`clip: true` (3.10 s -> 0.13 s). Those were two changes at once. Isolating the second one - the same
row, plus one line - gives most of it, and the mechanism is now visible.

`scroll`, 20 000 rows, the identical `RecordRow`; `ClippedRecordRow` is `RecordRow { clip: true; }`:

| | `layout` | `clipped` |
| --- | --- | --- |
| **paragraph shapes, first frame** | **60 081** | **66** |
| first frame | 620.08 ms | **25.31 ms** |
| allocations, first frame | 1 641 941 | 224 271 |
| allocations per row | 82.1 | 11.2 |
| steady-state frame | 23.60 / 19.47 ms | 15.24 / 13.80 ms |
| RSS at 50 000 rows | 257.0 MB | 199.1 MB |

**60 081 is `3 x 20 000 + 81`** - every `Text` of every row, on screen or not. 66 is the rows actually
visible. One property removes the other 60 015.

## The mechanism, from the source

`clip: true` is not a hint about text measurement. It is **the only subtree pruning the renderer
has**, and it is one early return:

```rust
// item_rendering.rs:565-582, Clip::render
if clip_item.clip() {
    let clip_region_valid = self.combine_clip(..);
    // If clipping is enabled but the clip element is outside the visible range, then we don't
    // need to bother doing anything, not even rendering the children.
    if !clip_region_valid {
        return RenderingResult::ContinueRenderingWithoutChildren;
    }
}
```

An off-screen clipped row returns before its children are visited. Its three `Text` items are never
reached, their geometry is never read, and reading a `Text`'s geometry inside a layout is what runs
the layout and therefore shapes the text.

The timing confirms where this happens: the shaping is **not** in the initial layout pass.

| row style | `show + layout` | first frame |
| --- | --- | --- |
| `layout` | 37.90 ms | **576.07 ms** |
| `clipped` | 33.42 ms | **19.85 ms** |

`show + layout` is the same for both. The 60 081 shapes happen during the **first render**, driven by
the render traversal reading every item's geometry - which is candidate 35's traversal, and the same
one candidates 12 and 32 want to prune.

### So this is the subtree-bounds fix, done by hand

Candidates 12, 13, 15 and 32 all want the renderer to skip subtrees that are off screen, and all
record that it cannot, because a child may draw outside its parent. `clip: true` is the developer
promising that it will not - and Slint already has the machinery to exploit that promise. **What is
missing is not the pruning but a way to know a subtree is bounded without being told.**

That reframes candidate 12: the prize is not hypothetical, it is measurable in-tree today by adding
one property, and it is **24x on startup and -45% per frame** at 20 000 rows.

## Which part of the row triggers it: the layout itself, not stretch and not elide

The obvious suspects are `horizontal-stretch: 1` (the layout has to distribute space, so it needs the
children's preferred widths) and `overflow: elide` (eliding needs to know how wide the text would
be). Both are wrong. Added a `layout-fixed` row style - the same `HorizontalLayout`, every child a
fixed width, no stretch anywhere, no elide anywhere:

| row style | first frame | allocations | paragraph shapes |
| --- | --- | --- | --- |
| `layout` - stretch + elide | 588.66 ms | 1 641 942 | **60 081** |
| `layout-fixed` - fixed widths, no stretch, no elide | 503.69 ms | 1 581 478 | **60 081** |
| `flat` - explicit x/y/width, **with** elide | 23.81 ms | 143 629 | **66** |
| `clipped` - layout + `clip: true` | 21.13 ms | 224 274 | **66** |

`layout-fixed` still shapes all 60 081. `flat` keeps `overflow: elide` and shapes 66.

**So the trigger is having a layout in the row at all.** A `HorizontalLayout` computes its own
layout-info from its children whether or not it has anything left to solve, and a `Text`'s
layout-info is only known after shaping. Removing the stretch does not remove the question, it only
removes what the answer is used for.

Two escapes, both app-level and both worth ~24x at startup:

1. **Position the row's children explicitly** (`flat`) - nothing asks the children for their size.
2. **`clip: true` on the row** (`clipped`) - keeps the layout, and the shaping still does not happen.

## What it is worth

- **24x on startup** (620 ms -> 25 ms at 20 000 rows), and it scales with the row count.
- **7x fewer allocations** at startup.
- **-35% per frame** in steady state, and -23% RSS at 50 000 rows.
- It is **one line in the .slint file** and needs no change to Slint.

The caveat is what `clip` actually does: content that overflows the row is cut off. For a row of
elided text that is exactly the intent, so the two go together naturally - but it is not free of
visual consequences, and that is why it cannot simply be the default.

## A harness bug found on the way

The `clipped` branch of `scroll-mode.slint` did not set `variant: index`, which the `layout` branch
does - so every "clip is -40%" number in this workspace had been comparing two changes rather than
one. Fixed. Re-measured afterwards: **it changed nothing** (614.35 -> 614.35 ms and 28.51 -> 28.51 ms
per row style), because `variant` only feeds the font size and `--font-sizes` defaults to 1, which
folds it to a constant. The conclusion survived, but that was luck rather than method - the
comparison should have been one change from the start.

## Reproduce

```sh
for st in layout clipped; do
  perf_test --bench --mode scroll --count 20000 --row-style $st --frames 3
done
# read the `paragraph shapes` column of frame 0
```
