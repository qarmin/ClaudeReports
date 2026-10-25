# circle_ellipse_visual_bounding_rect_half_stroke

`CircleShape::visual_bounding_rect` (`crates/epaint/src/shapes/circle_shape.rs:35`) and `EllipseShape::visual_bounding_rect` (`ellipse_shape.rs:59`), documented as "includes stroke widths". Found by comparing every shape's `visual_bounding_rect` with the bounds of the mesh the tessellator actually produces for it (random shapes, feathering and culling off).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Bounds comparison | 20000 random circles, ellipses, rects (all stroke kinds, rotated, blurred), line segments, paths; mesh bounds minus `visual_bounding_rect`, release | circle and ellipse draw up to half a stroke width (9.99 of width 19.98) beyond their bounding rect; rect, rotated rect, blur and line segments stay within 0.7 (pixel snapping); open/closed paths overshoot too (see "not done") |
| Standalone repro | circle radius 10 and ellipse radii 10x20 with a stroke of width 20, `visual_bounding_rect` vs the tessellated mesh | bounding rect half-extent 20 but the stroke reaches 30 (circle) and 30.09 (ellipse): 10 points too small |

## Status

**Confirmed** - deterministic geometry, 3/3 runs, debug build.

**Fully reproducible**: **Yes** (public `Shape::visual_bounding_rect` against the public `Tessellator`).

## Class

logic

## Root cause

`tessellate_circle` and `tessellate_ellipse` draw the stroke with `PathStroke::from(stroke).outside()`, i.e. the full stroke width lies outside the radius, but both bounding rects add only `stroke.width` to the diameter (half the width per side, the centered-stroke formula).

## Introduced by

cf7150c6a "Refactor: put each shape into its own file (#5564)" (circle, a move) and bce257adb "`epaint`: Add `EllipseShape` (#4122)" (ellipse) are the oldest commits `git log -S` finds; the circle formula predates the move (not determined further).

## The fix

Use `radius * 2 + stroke.width * 2` (`patch.diff`, +4/-2 over both files). After the patch the circle is exact and the random check shows 0.00 overshoot for circles; ellipses overshoot by at most 0.83 in the random run (miter offsets at the tight ends of thin ellipses; a separate, much smaller effect).

- Project test suite (`cargo test -p epaint --lib --features default_fonts`): 95 passed / 0 failed before -> same after.

## What's not done / open questions

- `PathShape::visual_bounding_rect` ignores `PathStroke::kind` (it always adds half the width) and miter joins, so `StrokeKind::Outside` paths and sharp corners are drawn outside it; tessellate_path culls with that rect, so a shape can be culled while part of its stroke is still inside the clip rect. Not reproduced end to end here; left in `IDEAS.md`.
- `Tessellator::tessellate_circle`/`tessellate_ellipse` do their own culling (`clip_rect.expand(radius + stroke.width).contains(center)`), which is correct, so the wrong rect mainly affects callers of the public API (bounding boxes for layout, hit boxes, the paint-bezier demo).
