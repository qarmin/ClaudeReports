# 09 - Gradient paints allocated a Vec of stops per primitive per frame

**Status: win, applied to the tree, validated.** Plan: `PLAN.md`.

## The change (`gradient_stops_iterator.diff`)

`GLItemRenderer::brush_to_paint` collected the gradient stops into a `Vec` for every
gradient-filled primitive of every frame, only so that it could look at the last stop and append
an extra one at position 1.0. femtovg's `linear_gradient_stops` / `radial_gradient_stops` take
`impl IntoIterator`, so the collect was pure overhead.

Replaced by a helper that walks the stops iterator twice (once for the last stop, once for the
values) and chains the extra stop - no allocation. That needed one widening in `i-slint-core`:
`LinearGradientBrush::stops()`, `RadialGradientBrush::stops()` and the conic one now return
`impl Iterator<Item = &GradientStop> + Clone` (they already returned a `Skip<slice::Iter>`
underneath, so nothing changed but the advertised bound).

## Measured (FemtoVG under Xvfb, `rects` scene with `--gradient`, 3 000 rectangles, ~1 100 visible)

| | baseline | patched |
| --- | --- | --- |
| allocations per frame | 58 002 | **56 888** |
| frame time (median of 3) | 52.05 ms | 51.58 ms |

-1 114 allocations per frame = exactly one per visible gradient rectangle. The time difference
(-0.9%) is inside the llvmpipe noise; at 13 ns per allocation the saving is ~15 us per frame,
which matters on a GPU-bound frame of a few milliseconds, not on a 52 ms software-rasterized one.

```sh
just run          # this folder
```

## Validation

- Pixel comparison, gradient scene moving 31 px/frame for 4 frames: **byte-identical**.
- `cargo test -p i-slint-core`: 361 passed. `cargo test -p slint`: 77 passed.
  `tests/`: `test-driver-interpreter` 818 passed. All 0 failed.

## Follow-up found while measuring, not fixed here

The allocation ranking for the gradient scene also shows, per gradient primitive per frame:

- `GradientColors::from_stops` -> `to_rc_slice` (femtovg): a fresh `Rc<[GradientStop]>` per paint,
  ~1 116 per frame. femtovg keeps `MultiStopGradient { shared_stops: Rc<[GradientStop]>, tint }`
  and caches the generated gradient *texture* in a `BTreeMap`, so only the stop list is
  reallocated. An inline small-stop representation (2-4 stops covers most UI gradients) would
  remove it, at the cost of touching `Ord`/`Eq` for the cache key.
- `femtovg::path::cache::{Point, Contour}` grow by **1 110 more** per frame than in the
  solid-color version of the same scene - one extra path tessellation per gradient primitive.
  That is `path_bounding_box()` (`itemrenderer.rs:155-170`) asking femtovg for the path's bbox
  with the transform reset, which misses the path's tessellation cache. See candidate 10.
