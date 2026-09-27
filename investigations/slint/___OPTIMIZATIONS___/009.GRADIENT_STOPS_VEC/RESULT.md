# 09 - Gradient paints allocated a Vec of stops per primitive per frame

Ported from `perf_test/___OPTIMIZATIONS___/09_gradient_stops_vec/RESULT.md` on 2026-08-23 (original perf_test-relative session; source and any extra notes remain there untouched).

Applied to the tree, validated. Plan: `PLAN.md`.

## What was tried (`patch.diff`)

`GLItemRenderer::brush_to_paint` collected the gradient stops into a `Vec` for every
gradient-filled primitive of every frame, only so that it could look at the last stop and append
an extra one at position 1.0. femtovg's `linear_gradient_stops` / `radial_gradient_stops` take
`impl IntoIterator`, so the collect was pure overhead.

Replaced by a helper that walks the stops iterator twice (once for the last stop, once for the
values) and chains the extra stop - no allocation. That needed one widening in `i-slint-core`:
`LinearGradientBrush::stops()`, `RadialGradientBrush::stops()` and the conic one now return
`impl Iterator<Item = &GradientStop> + Clone` (they already returned a `Skip<slice::Iter>`
underneath, so nothing changed but the advertised bound).

## Result

Measured (FemtoVG under Xvfb, `rects` scene with `--gradient`, 3 000 rectangles, ~1 100 visible):

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

## Does it change behavior/output?

- **Bit-exact?** Yes. Pixel comparison, gradient scene moving 31 px/frame for 4 frames:
  byte-identical.
- **Is this a breaking change?** No - the only public-facing change is that `stops()` on the
  gradient brush types now returns `impl Iterator<...> + Clone` instead of a concrete
  `Skip<slice::Iter>` type; the underlying values and behavior are unchanged.
- Project test suite: `cargo test -p i-slint-core`: 361 passed. `cargo test -p slint`: 77 passed.
  `tests/`: `test-driver-interpreter` 818 passed. All 0 failed.

## Why the result looks the way it does

The `Vec` collection existed only so the code could peek at the last stop before appending an
extra terminal stop at position 1.0; femtovg's gradient-stop APIs already accept any
`IntoIterator`, so replacing the collect with a two-pass iterator walk (once to find the last
stop, once to chain it) removes the allocation entirely without changing what gets drawn.

## What's not done / open questions

The allocation ranking for the gradient scene also shows, per gradient primitive per frame:

- `GradientColors::from_stops` -> `to_rc_slice` (femtovg): a fresh `Rc<[GradientStop]>` per paint,
  ~1 116 per frame. femtovg keeps `MultiStopGradient { shared_stops: Rc<[GradientStop]>, tint }`
  and caches the generated gradient *texture* in a `BTreeMap`, so only the stop list is
  reallocated. An inline small-stop representation (2-4 stops covers most UI gradients) would
  remove it, at the cost of touching `Ord`/`Eq` for the cache key. Not attempted here.
- `femtovg::path::cache::{Point, Contour}` grow by **1 110 more** per frame than in the
  solid-color version of the same scene - one extra path tessellation per gradient primitive.
  That is `path_bounding_box()` (`itemrenderer.rs:155-170`) asking femtovg for the path's bbox
  with the transform reset, which misses the path's tessellation cache. See candidate
  `010.GRADIENT_BBOX_TESSELLATION`.

## Reproduce

```sh
just run
```

Fully reproducing the bench additionally needs `perf_test/` copied in manually, since it is
untracked in the main repo's git history.

## Reproducibility check after upstream update (2026-08-23)

`patch.diff` no longer applies cleanly against slint HEAD `cdb6c43f7`, and unlike 003/006 this is not just line drift: femtovg gradient handling was substantially refactored upstream since this patch was captured. `internal/renderers/femtovg/itemrenderer.rs` now resolves brushes through a `ResolvedBrush` enum and a `to_femtovg_stops(&[GradientStop]) -> Vec<(f32, femtovg::Color)>` helper (see `itemrenderer.rs:1404-1437`), rather than calling `Brush::stops()` directly at the renderer call site the way this patch assumed. `brush.rs::stops()` itself is unchanged (still `impl Iterator<Item = &GradientStop>` without `+ Clone`), but nothing in `itemrenderer.rs` calls it anymore - the allocation this patch targeted may have moved, changed shape, or been absorbed into `ResolvedBrush`'s own construction. **Needs a fresh look at `ResolvedBrush`/`to_femtovg_stops` to determine whether the original inefficiency (an allocation per gradient-filled primitive per frame) still exists in the new code path, not a mechanical rebase of this diff.** Not investigated further this round - flagged as the next thing to look at, added to `IDEAS.md`.
