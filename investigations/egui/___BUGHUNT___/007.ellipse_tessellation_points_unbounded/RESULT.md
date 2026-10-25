# ellipse_tessellation_points_unbounded

`Tessellator::tessellate_ellipse` in `crates/epaint/src/tessellator.rs:1691`. Found by a randomized tessellator probe that hung on ellipses with very large radii; looked at the point-count formula `max(8, radius * pixels_per_point / 16)`, which has no upper bound (a `CircleShape` by contrast always uses at most 128 points).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Randomized tessellation probe | 3 s watchdog per shape, huge radii 1e5-1e20 | ellipses with huge radius never finish |
| Standalone repro (public API) | `Shape::ellipse_stroke` with centre inside the clip rect, ppp 1, radius 100 / 1e4 / 1e6 / 1e8 / 1e9, release build, `ulimit -v 6000000`, each radius in a child process with a 20 s watchdog | 128 / 10000 / 1,000,000 / 100,000,000 vertices (1.8 s), and for 1e9 the process aborts: `memory allocation of 4000000000 bytes failed` |

## Status

**Confirmed** - reproduces 3/3 runs (release build; the vertex counts are exact and deterministic, timings approximate).

**Fully reproducible**: **Yes** (public `Shape` + `Tessellator::tessellate_shapes`, same path the egui painter uses).

## Class

crash

## Root cause

`num_points = max(8, (radius.max_elem() * pixels_per_point) as u32 / 16)` points per quarter are generated, and the only culling (`coarse_tessellation_culling`) tests the ellipse centre against the clip rect expanded by the radius, so a huge ellipse that covers the screen is never culled. Vertex count (and the temporary `Vec<Vec2>`/`Vec<Pos2>` buffers) therefore grows linearly with the radius, up to `u32::MAX / 16` per quarter: hundreds of millions of vertices, then an allocation failure abort.

## Introduced by

bce257adb "`epaint`: Add `EllipseShape` (#4122)" added the formula.

## The fix

Clamp the per-quarter point count to `8..=1024` (`patch.diff`, +2/-1). Radii up to ~16000 px (any realistic on-screen ellipse) tessellate exactly as before; after the patch radius 1e6, 1e8 and 1e9 all produce 16384 vertices in well under a millisecond.

- Project test suite (`cargo test -p epaint --lib --features default_fonts`): 95 passed / 0 failed before -> same after.

## What's not done / open questions

- `Path::add_circle_quadrant`/rounded rects use fixed point counts so they are not affected; bezier curves with huge control points also hang in the probe but were not analysed here.
- 1024 per quarter is a judgement call; a maintainer may prefer a different cap or clipping the ellipse to the clip rect.
