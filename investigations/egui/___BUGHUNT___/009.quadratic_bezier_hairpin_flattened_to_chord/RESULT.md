# quadratic_bezier_hairpin_flattened_to_chord

`QuadraticBezierShape::for_each_flattened_with_t` / `FlatteningParameters::from_curve` in `crates/epaint/src/shapes/bezier_shape.rs` (~490, ~540). Lead from candidate 008's investigation: a collinear quadratic was flattened to the chord of its end points although the curve overshoots them; probing showed it is not limited to the collinear case.

## What was tried

| Approach/tool | Config (build/input/duration) | Outcome |
|---|---|---|
| Hand-built hairpins | quadratic [0,0] [10,dy] [1,0], dy in 0 .. 3, tolerance 0.1 | for dy <= 1 the result is the 2-point chord [0,0]-[1,0] although the curve reaches x = 5.263 (deviation 4.29 for a tolerance of 0.1); dy = 3 is correct |
| Real tessellator | the same curve as a `Shape::QuadraticBezier` with a 2 px stroke through `Tessellator::tessellate_shapes` | stroke vertices only reach x = 2.0, the curve itself reaches 5.263 |
| Randomized check | 200000 random quadratics, coordinates in [-100, 100], tolerance 0.1, max distance of the true curve (201 samples) from the flattened polyline, release | 880 curves (0.44%) deviate by more than 2x the tolerance, worst deviation 107.9 (e.g. [-85.8,53.2] [83.7,-82.4] [-87.8,54.8] is flattened to 2 points) |
| Count analysis | worked `FlatteningParameters::from_curve` by hand for dy = 1 | `count = ceil(0.5 * 25.7 * sqrt(5.7e-4 / 0.1)) = 1`: the formula underestimates for arcs that contain the apex with the end points far on either side |

## Status

**Confirmed** - deterministic (fixed inputs and a fixed-seed generator), identical on 3/3 runs, release build (debug gives the same numbers).

**Fully reproducible**: **Yes** (public `QuadraticBezierShape::flatten` and the real `Tessellator`, no second participant).

## Class

logic

## Root cause

The flattening algorithm (Levien's parabola-integral subdivision, ported from lyon) picks the number of segments from the integral between the two ends of the arc. For a hairpin, an arc that turns by more than a right angle with its apex inside the arc, the estimate is far too low (often 1), so the curve is replaced by one chord or by badly placed points. The same shape is produced for every `QuadraticBezierShape` stroke/fill, and for `CubicBezierShape` pieces, which are converted into quadratics first.

## Introduced by

041f2e64b "Move all crates into a `crates` directory (#1940)" is where `git log -S` stops (a move); the ported algorithm is older (not determined further).

## The fix

Split a quadratic that turns by more than 90 degrees (`(ctrl - from) . (to - ctrl) < 0`) at the apex of its parabola and flatten each half with the existing code (`patch.diff`, +48/-7, of which 7 lines are test expectations). After the patch the randomized check reports 0 of 200000 curves deviating by more than 2x tolerance (was 880), and the hand-built hairpins reach x = 5.263.

- Side effect: such curves now contain one more point (the apex), so 7 `assert_eq!(result.len(), N)` expectations in `bezier_shape.rs` tests went up by exactly 1 (25->26, 25->26, 71->72, 11->12, 24->25, 72->73, 223->224); this is part of the patch.
- Project test suite (`cargo test -p epaint --lib --features default_fonts`): 95 passed / 0 failed before -> 95 passed / 0 failed after (with the 7 updated expectations; without them 5 tests fail by +1 point).
- A cheaper variant (split only when the flattening count is <= 1) passed the existing tests unchanged but left 729 of 200000 violations (worst 2.18), so it was not used; a variant with count <= 8 left 0 violations and changed 1 expectation.

## What's not done / open questions

- Cubic curves with cusps/hairpins were not checked separately (they reduce to quadratics, so likely improved, but no measurement was made).
- A maintainer may prefer a threshold-based split (fewer changed point counts) or a different apex criterion.
