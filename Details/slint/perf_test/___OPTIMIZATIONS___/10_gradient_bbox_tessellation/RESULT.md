# 10 - Gradients tessellated every path twice (bounding box with the transform reset)

**Status: win, applied to the tree, validated.**

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

## The change (`brush_to_paint_with_bounds.diff`)

(That diff is against git HEAD, so it also contains candidate 09's `gradient_stops` helper -
both are applied to the tree and touch the same file.)

Split `brush_to_paint` in two:

- `brush_to_paint_with_bounds(brush, path_bounds)` - the body, taking the reference box.
- `brush_to_paint(brush, path)` - unchanged behaviour for callers that only have a path
  (`Path` elements), and it now skips the measuring entirely for solid colors, which never look
  at the bounds.

Every call site that builds its path from a rectangle it already has passes that rectangle:
`draw_rectangle`, `draw_border_rectangle` (background and border bounds are different - the
border rect is the one adjusted for inner drawing), the text fill/stroke brushes, and the image
colorize brush.

## Measured (FemtoVG under Xvfb, `rects` 3 000, medians of 3)

| scene | allocations/frame before | after | time before | after |
| --- | --- | --- | --- | --- |
| `rects --gradient` | 56 888 | **51 304 (-9.8%)** | 52.62 ms | **50.69 ms (-3.7%)** |
| `rects` (solid) | 50 227 | 50 193 | 45.72 ms | 45.02 ms |

-5 584 allocations per frame for ~1 110 gradient rectangles = **5 fewer allocations per gradient
primitive** (the tessellation's `Point`, `Contour`, and the vertex/bounds vectors it pulls in).
Together with candidate 09 the gradient scene went from 58 002 to 51 304 allocations per frame
(-11.5%).

## Validation

- Pixel comparison of the gradient rect scene moving 31 px/frame: **byte-identical**.
- Gradient **text** (`color: @linear-gradient(...)` on the list rows, which goes through
  `platform_text_fill_brush`): differs from the baseline in 9 114 bytes - but two runs of the
  *same* binary differ in 9 132 bytes, and the differing pixels are rows 839-851, which is the
  status line printing per-run timings. So no difference attributable to the change. (Methodology
  note: the status line makes any perf_test scene non-comparable pixel-wise; compare above it or
  use the `rects` scene.)
- `cargo test -p i-slint-core` 361, `cargo test -p slint` 77, `tests/` interpreter driver 818,
  Rust driver 745 - all pass, 0 failures.

## Follow-up

`path_bounding_box` is still used by `Path` element fills/strokes and by the clip-path bounds in
`visit_clip`. For `Path` elements the double tessellation remains; femtovg could expose a
transform-independent bbox (it already stores untransformed verbs) or Slint could cache the bbox
per item. Not attempted here.
