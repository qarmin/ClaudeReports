# Precomputed outline normals for rounded rectangles with four equal corners

Rounded rectangles are the most common widget shape, and `Path::add_line_loop` (a `normalized()` plus a miter divide per outline point, 12-132 points) was 31% of `shape_rect_fill_round4_1000` and 22% of the stroked variant. For a rect whose four corners have the same radius and no merged corners, the outline normals depend only on the corner shape (table resolution), not on the radius or position. `tessellate_rect` now takes them from a `OnceLock` table computed once per resolution (12/20/36/68/132 points, via `add_line_loop` on a canonical rectangle) instead of computing them for every rectangle. Falls back to `add_line_loop` for rotated rects, unequal radii, radius < 0.5, or merged corners (point count not in the table).

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time.

## What was tried

Interleaved A/B of prebuilt scenes-harness binaries (`tools/ab.sh`, pinned core, 3 rounds). Ratios patched/base:

| Benchmark | ratios |
|---|---|
| shape_rect_fill_round4_1000 (1000 filled rects, radius 4) | 0.6191, 0.6144, 0.6211 |
| shape_rect_stroke_inside_1000 (radius 4, stroke 1) | 0.7230, 0.7174, 0.7255 |
| shape_rect_fill_stroke_round_1000 (radius 6, stroke 1.5, 40x20) | 0.7327, 0.7272, 0.7384 |
| scene_buttons_300 | 0.8345, 0.8418, 0.8346 |
| scene_widget_mix_100 | 0.9210, 0.9158, 0.9218 |
| scene_windows_30 | 0.9315, 0.9299, 0.9350 |

## Result

Time: filled rounded rect tessellation -38%, stroked rounded rects -27 to -28%, a column of 300 buttons -16.5%, mixed widget scene -8%, windows -7% (3/3 rounds each).

## Does it change behavior/output?

- **Bit-exact?** No, characterized difference. The table normals are those of a canonical rectangle, whereas `add_line_loop` derives them from the actual float positions, which carry coordinate rounding noise. Measured over 166,164 random qualifying rectangles (sizes 4-404 px, positions -1000..3000, radius 0.5-80) the largest component difference between table normal and per-rect normal is 3.2e-4, and 6.8e-5 for the typical UI range (radius >= 2, |x|,|y| < 1000). Normals are unit-scale and scale the 1-pixel feathering/half stroke width, so vertices move by < 1e-4 px (< 4e-4 px in the extreme sample); in the noisy extremes (small radius at large coordinates) the table normal is closer to the exact shape than the old value. The equivalence snippet is in `equivalence_test_snippet.rs`.
- Triangle topology, vertex counts and colors are unchanged.
- Is this a breaking change? No API change. Anything pixel-comparing tessellated meshes bit-for-bit (snapshot tests with 0 tolerance) could see sub-0.0001 px vertex differences; `cargo test -p epaint -p egui --all-features` passes (93 + 3 egui, 188 integration, 95 + 4 epaint); the egui_demo_lib snapshot tests fail in this environment even on the base commit, so they could not be used as a pixel oracle.
- Real change to the tested project's source: `patch.diff` (+45/-1).

## Why the result looks the way it does

Per rounded rectangle, `add_line_loop` did 12-20 sqrt + 4 divides + loop overhead; the table lookup replaces it with a copy. Removing it also shrinks the share of tessellation in whole-UI scenes.

## What's not done / open questions

- Unequal radii (e.g. only top corners rounded) still use the slow path; per-corner tables could cover them.
- The same idea applies to `Shape::Ellipse`/other regular outlines built from the circle tables.
- A maintainer may prefer bit-exactness; the table could instead be validated against `add_line_loop` in a debug assertion.
