# closed_path_two_points_nan_vertices

`Path::add_line_loop` in `crates/epaint/src/tessellator.rs:432`, used for every closed `Shape::Path` and `Shape::convex_polygon`. Found by a randomized tessellator probe (random shapes, finite sane coordinates, check that all output vertices are finite) after it reported non-finite vertices for a closed 2-point path.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Randomized tessellation probe | 20000 shapes (rect/circle/ellipse/path/beziers/polygon, finite sane coords, random strokes/kinds/ppp), release | panics: none; non-finite vertices only for closed paths (56 of 3000 in the first run) |
| Hand-built variants | closed paths: 2 points, 3 collinear, repeated point, all-equal; all three `StrokeKind`s, feathering on/off, filled/unfilled | every path that doubles back on itself (180 degree turn) gives NaN vertices, open paths and ordinary polygons do not |
| Zero-size `RectShape` | 0x0, 0xH, Wx0 rects with stroke, all kinds, corner radius 0/4, ppp 1/1.5 | all finite (rects are not affected) |
| End-to-end | `Shape::closed_line` / `Shape::convex_polygon` through `Tessellator::tessellate_shapes` | NaN vertex positions in the output `Mesh` (debug and release) |

## Status

**Confirmed** - reproduces 3/3 runs (debug and release), deterministic.

**Fully reproducible**: **Yes** (public `Shape` + `Tessellator::tessellate_shapes`, the same path egui's painter uses).

## Class

logic

## Root cause

For a closed path the miter normal at each vertex is the average of the incoming and outgoing edge normals divided by its own squared length. When the path turns exactly back on itself (a 2-point loop, or collinear points) the two normals cancel, `length_sq == 0` and `normal / length_sq` is `0.0 / 0.0 = NaN`. The NaN is written into the stroke (and feather) vertices; all points equal gives NaN too because both normals are zero. The open-path code does not hit this because it cuts off sharp corners first.

## Introduced by

041f2e64b "Move all crates into a `crates` directory (#1940)" is only where `git log -S` stops (a pure move); the code predates it (not determined further).

## The fix

If `length_sq < 1e-6` (turning back within ~0.1 degrees) use the incoming edge normal instead of the miter (`patch.diff`, +3/-0). After the patch every case of the repro has 0 non-finite vertices, and the randomized probe reports 0 non-finite vertices over 20000 shapes.

- Project test suite (`cargo test -p epaint -p egui --lib --features epaint/default_fonts`): 86 + 95 passed / 0 failed before -> same after.

## What's not done / open questions

- Near-reversals slightly above the threshold still give very long (finite) miter spikes; that is existing behaviour, not looked at.
- Whether the NaN triangles are visibly wrong on a real GPU backend was not checked (backend dependent); the output mesh itself contains NaN positions.
- A debug-build probe with non-finite or huge inputs hangs the tessellator (see the notes in the IDEAS backlog), not part of this candidate.
