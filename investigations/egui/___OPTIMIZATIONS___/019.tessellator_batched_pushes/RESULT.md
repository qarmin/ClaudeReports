# Batch vertex/index pushes in the tessellator stroke and feather loops

`stroke_and_fill_path` (40%) and `add_open_points` (28%) dominate a long polyline (`scene_polylines_2x5000_points`); the stroke loop pushes 4 vertices (`colored_vertex`) and 6 triangles (`add_triangle`, 3 `extend_from_slice` calls each) per path point. Replaced them in the closed/open thick-stroke branches and in `fill_closed_path`'s feathering loop by one `extend_from_slice` for the vertices and one for all indices (new helper `push_strip_quads`), hoping to cut per-push capacity checks.

rustc 1.95.0 (59807616e 2026-04-14), base commit 86099aca1. Metric: wall-clock time.

## What was tried

Interleaved A/B (patched/base, 4 rounds), scenes harness: scene_polylines_2x5000_points 1.0078, 1.0059, 0.9883, 1.0003; scene_painter_shapes_2000 0.9986, 0.9994, 1.0072, 1.0123; scene_widget_mix_100 1.0017, 1.0007, 1.0011, 1.0075 (the printed order is by criterion, polylines first). demo_only_tessellate: 0.9785, 1.0012, 0.9806, 1.0127.

## Result

Within noise: no consistent change. LLVM already hoists the capacity checks out of the loop because `reserve_*` is called before it. Time only.

## Does it change behavior/output?

- **Bit-exact?** Yes (same values in the same order; only the debug-only `colored_vertex` assertion on the mesh texture is no longer executed in these loops).
- Breaking change: no.
- Tests: `cargo test -p epaint --all-features` 95 + 4 passed (1 ignored), same as unpatched.
- Real change to the tested project's source: `patch.diff` (+64/-42).

## Why the result looks the way it does

The remaining stroke cost is arithmetic on the vertex positions (normal scaling, colour closure) and memory writes, not push bookkeeping; the profile's hot lines in `stroke_and_fill_path` (`mod.rs:547`) were the vector stores themselves.

## What's not done / open questions

- Reducing divisions in `add_open_points` / `normalized()` would change float results by an ULP (characterized-difference territory); not tried here.
