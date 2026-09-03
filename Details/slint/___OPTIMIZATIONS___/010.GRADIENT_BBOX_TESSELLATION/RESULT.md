# 10 - Gradients tessellated every path twice (bounding box with the transform reset)

Ported from `perf_test/___OPTIMIZATIONS___/10_gradient_bbox_tessellation/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Applied to the tree, validated.

## Cause

`brush_to_paint` needs the path's bounding box to place a gradient. It got it from
`path_bounding_box()` (`internal/renderers/femtovg/itemrenderer.rs:172-188`), which does

```rust
canvas.save(); canvas.reset_transform();
let bounding_box = canvas.path_bbox(path);
canvas.restore();
```

femtovg's `path_bbox` builds the path's tessellation cache, and that cache is **keyed by the
transform** (`path.cache(&transform, tess_tol, dist_tol)`). Resetting the transform therefore
tessellates the path once for the bbox and again, with the real transform, for the fill: **two
tessellations per gradient-filled primitive per frame**, each allocating its `Point`/`Contour`
vectors.

Visible in the allocation ranking as `femtovg::path::cache::{Point, Contour}` growing by ~1 110
extra allocations each per frame in the gradient version of a 3 000-rectangle scene with ~1 110
visible rectangles - exactly one extra tessellation per rectangle.

## What was tried (`patch.diff`)

(That diff is against git HEAD in the original session, so it also contains candidate
`009.GRADIENT_STOPS_VEC`'s `gradient_stops` helper - both are applied to the tree and touch the
same file.)

Split `brush_to_paint` in two:

- `brush_to_paint_with_bounds(brush, path_bounds)` - the body, taking the reference box.
- `brush_to_paint(brush, path)` - unchanged behaviour for callers that only have a path
  (`Path` elements), and it now skips the measuring entirely for solid colors, which never look
  at the bounds.

Every call site that builds its path from a rectangle it already has passes that rectangle:
`draw_rectangle`, `draw_border_rectangle` (background and border bounds are different - the
border rect is the one adjusted for inner drawing), the text fill/stroke brushes, and the image
colorize brush.

## Result

Measured (FemtoVG under Xvfb, `rects` 3 000, medians of 3):

| scene | allocations/frame before | after | time before | after |
| --- | --- | --- | --- | --- |
| `rects --gradient` | 56 888 | **51 304 (-9.8%)** | 52.62 ms | **50.69 ms (-3.7%)** |
| `rects` (solid) | 50 227 | 50 193 | 45.72 ms | 45.02 ms |

-5 584 allocations per frame for ~1 110 gradient rectangles = **5 fewer allocations per gradient
primitive** (the tessellation's `Point`, `Contour`, and the vertex/bounds vectors it pulls in).
Together with candidate `009.GRADIENT_STOPS_VEC` the gradient scene went from 58 002 to 51 304
allocations per frame (-11.5%).

## Does it change behavior/output?

- **Bit-exact?** Yes for `rects` (solid and gradient): pixel comparison of the gradient rect
  scene moving 31 px/frame is byte-identical. Gradient **text**
  (`color: @linear-gradient(...)` on the list rows, which goes through
  `platform_text_fill_brush`): differs from the baseline in 9 114 bytes - but two runs of the
  *same* binary differ in 9 132 bytes, and the differing pixels are rows 839-851, which is the
  status line printing per-run timings. So no difference attributable to the change. (Methodology
  note found here: the status line makes any perf_test scene non-comparable pixel-wise; compare
  above it or use the `rects` scene.)
- **Is this a breaking change?** No - internal refactor of `brush_to_paint`'s call sites, no
  public API or documented behavior change.
- Project test suite: `cargo test -p i-slint-core` 361, `cargo test -p slint` 77, `tests/`
  interpreter driver 818, Rust driver 745 - all pass, 0 failures.

## Why the result looks the way it does

`path_bbox()` resets the canvas transform before asking femtovg for the bounding box, and
femtovg's tessellation cache is keyed by transform - so the reset forces a second, throwaway
tessellation purely to measure the box, before the real one for drawing. Passing the caller's
already-known rectangle bounds (`path_bounds`) instead of asking femtovg to compute them from the
path skips that second tessellation entirely, and skipping the measurement altogether for solid
colors (which never consult the bounds) removes it there too.

## What's not done / open questions

`path_bounding_box` is still used by `Path` element fills/strokes and by the clip-path bounds in
`visit_clip`. For `Path` elements the double tessellation remains; femtovg could expose a
transform-independent bbox (it already stores untransformed verbs) or Slint could cache the bbox
per item. Not attempted here.

## Reproduce

```sh
just run          # this folder: gradient vs solid, allocations/frame
```

Fully reproducing the bench additionally needs `perf_test/` copied in manually, since it is
untracked in the main repo's git history.

## Reproducibility check after upstream update (2026-08-23)

`patch.diff` no longer applies cleanly against slint HEAD `cdb6c43f7`, and unlike 003/006 this is not just line drift: femtovg gradient handling was substantially refactored upstream since this patch was captured. `internal/renderers/femtovg/itemrenderer.rs` now resolves brushes through a `ResolvedBrush` enum and a `to_femtovg_stops(&[GradientStop]) -> Vec<(f32, femtovg::Color)>` helper (see `itemrenderer.rs:1404-1437`), rather than calling `Brush::stops()` directly at the renderer call site the way this patch assumed. `brush.rs::stops()` itself is unchanged (still `impl Iterator<Item = &GradientStop>` without `+ Clone`), but nothing in `itemrenderer.rs` calls it anymore - the allocation this patch targeted may have moved, changed shape, or been absorbed into `ResolvedBrush`'s own construction. This candidate's own fix (avoid tessellating the gradient bbox twice, via a `brush_to_paint_with_bounds` that reuses an already-known rect) sits in the same function this refactor touched. **Needs the same fresh look as candidate 009 - determine whether `ResolvedBrush` construction still measures the bbox twice, not a mechanical rebase.** Not investigated further this round - flagged as the next thing to look at, added to `IDEAS.md`.
