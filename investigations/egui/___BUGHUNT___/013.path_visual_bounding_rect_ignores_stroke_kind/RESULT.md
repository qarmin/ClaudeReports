# path_visual_bounding_rect_ignores_stroke_kind

`PathShape::visual_bounding_rect` (`crates/epaint/src/shapes/path_shape.rs:69`), which `Tessellator::tessellate_path` uses to cull paths that do not intersect the clip rect. Lead from candidate 012's bounds comparison (paths drew outside their bounding rect).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Bounds comparison | random open/closed paths, all `StrokeKind`s, release | mesh exceeds `visual_bounding_rect` by up to 17.7 for stroke width 19.9 (open path) |
| Standalone repro (public API, real culling) | closed 10x10 square, stroke width 20, kinds Middle and Outside, clip rect x -18..-12, y 0..10 through `Tessellator::tessellate_shapes` | Middle: stroke covers x -10..20, no overlap with the clip rect, culled (correct). Outside: stroke covers x -20..30, which overlaps the clip rect, but the shape is culled (nothing tessellated) because the bounding rect is x -10..20 |

## Status

**Confirmed** - deterministic, 3/3 runs, debug build.

**Fully reproducible**: **Yes** (real `Tessellator::tessellate_shapes` with default options, i.e. coarse culling on).

## Class

logic

## Root cause

The bounding rect always expands the point bounds by `stroke.width * 0.5` (a centered stroke). `StrokeKind::Outside` strokes extend the full width to one side, so the bounds are too small by half the width and the culling test `!visual_bounding_rect().intersects(clip_rect)` removes paths whose outer stroke band is still visible.

## Introduced by

The expression comes from the original path shape and was not traced past the file moves (not determined).

## The fix

Expand by the full width for `Inside` and `Outside` (which side is "inside" depends on the winding order) and by half for `Middle` (`patch.diff`, +6/-1). After the patch the Outside case is tessellated.

- Project test suite (`cargo test -p epaint --lib --features default_fonts`): 95 passed / 0 failed before -> same after.

## What's not done / open questions

- Miter joins still draw outside the rect (open paths with sharp turns overshoot by up to 7.8 for width 19.9 after the patch; closed paths with near-reversals produce huge spikes, which the tessellator's `CUT_OFF_SHARP_CORNERS = false` comment and PRs #7554 and #7901 already discuss). Only the stroke-kind part is covered here.
- `BandShape`, bezier shapes and `RectShape` were not checked for the same culling mismatch (the random bounds run showed `RectShape` within 0.7 only).
