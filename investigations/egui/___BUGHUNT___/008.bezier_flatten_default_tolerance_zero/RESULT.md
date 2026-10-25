# bezier_flatten_default_tolerance_zero

`CubicBezierShape::flatten`/`flatten_closed` and `QuadraticBezierShape::flatten` in `crates/epaint/src/shapes/bezier_shape.rs` (~251, ~267, ~474). The default tolerance (`tolerance: None`) is `|p0.x - p_last.x| * 0.001`, which looked suspicious: it is zero whenever both end points have the same x (HEURISTICS: a divisor/tolerance derived from a difference that can be zero).

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Code reading | `flatten(None)` -> `num_quadratics` divides by `432 * tolerance^2` and asserts `tolerance > 0` | zero tolerance not guarded |
| Standalone repro (public API) | 3 curves, each in a child process with a 10 s watchdog and `ulimit -v 6000000`: cubic with different end x (control), cubic with equal end x, quadratic with equal end x; debug and release | control: 84 points. Cubic equal x: debug panics `the tolerance should be positive`, release never finishes (`err / 0 = inf` becomes a `u32::MAX` loop count). Quadratic equal x: returns 2 points (a straight line) for a visibly bent curve, in both builds |

## Status

**Confirmed** - reproduces 3/3 runs in each build mode (deterministic).

**Fully reproducible**: **Yes** (public `flatten` API; the tessellator itself passes `Some(tolerance)` and is not affected, no in-repo caller of `flatten(None)` was found).

## Class

crash

## Root cause

The documented default tolerance is derived only from the x distance between the end points, so any curve whose end points share an x coordinate (a vertical arc) gets tolerance 0. Cubic: `num_quadratics` becomes `u32::MAX` (hang) or trips the debug assertion. Quadratic: `count` becomes infinite, the "not finite means a straight line" branch is taken and the curve is silently flattened to its end points.

## Introduced by

cf7150c6a "Refactor: put each shape into its own file (#5564)" is where `git log -S` stops (a move); the expression is older (not determined further).

## The fix

A helper `default_flatten_tolerance` keeps the old value when the x distance is positive, otherwise falls back to 0.1% of the control-point bounding box, with a floor of `f32::EPSILON` (`patch.diff`, +14/-3). After the patch: control unchanged (84 points), cubic equal-x gives 26 points, quadratic equal-x gives 21 points, in debug and release.

- Project test suite (`cargo test -p epaint --lib --features default_fonts`): 95 passed / 0 failed before -> same after.

## What's not done / open questions

- The related quadratic flattening problem (hairpin curves flattened to a chord) found while here is candidate 009.
- `to_path_shape(None)`/`to_path_shapes(None, ..)` go through the same code and were not run separately.
