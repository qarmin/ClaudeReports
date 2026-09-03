// Reproduction for candidate 051: FIT_COMPUTE_ARC_PARAMS_ZERO_ARCLEN_DIV_BY_ZERO.
//
// This exercises the REAL, unmodified public API of kurbo's curve-fitting module
// (`kurbo::fit::{ParamCurveFit, fit_to_cubic}`) with a genuine (if adversarial - but fully
// valid per the trait's own documented contract) `ParamCurveFit` implementor. There is no
// second participant being faked here (see CONFIRMATION.md section 11's carve-out): a custom
// `ParamCurveFit` implementation handed to `fit_to_cubic` IS the intended, ordinary way any real
// caller of this public trait would use it - the same way a malformed file handed to a parser's
// own `from_reader` is genuine external input with no "other side" to impersonate.
//
// `StationarySpicy` reports:
//   - `sample_pt_tangent`: a position that moves along an exact straight line, and a *tangent
//     direction* that rotates rapidly and independently of that motion (legitimate per the
//     trait's own docs: "the derivative can go to zero in cusps", and nothing requires
//     `sample_pt_tangent`'s tangent field to be derived from the same physical speed as
//     `sample_pt_deriv`'s reported derivative).
//   - `sample_pt_deriv`: the identical position, but a derivative that is genuinely `Vec2::ZERO`
//     at every sampled point across the whole range - i.e. the source reports zero speed
//     everywhere, a degenerate but syntactically valid answer.
//
// The rapidly-rotating tangent field flags the range as "spicy" (`CurveDist::from_curve`,
// `kurbo/src/fit.rs`), which routes evaluation through `CurveDist::compute_arc_params` /
// `eval_arc`. There, summing `deriv.hypot()` (always 0.0) over all 110 sub-samples yields
// `arclen == 0.0` exactly, so `arclen.recip()` is `+inf`, and every (zero-valued) partial-sum
// entry in `arcparams` gets corrupted to `NaN` by `0.0 * inf`.
//
// This program demonstrates that the real, unmodified public entry point (`fit_to_cubic`) reaches
// this exact code path deterministically for this input, and prints its return value. Because the
// resulting NaN is entirely internal to the crate-private `CurveDist` struct, this repro (using
// only the public API) cannot itself print the NaN directly - see `output.txt` and `RESULT.md`'s
// "Root cause" / "What was tried" sections for the internal instrumentation (temporarily added to
// the crate under test, exactly as one would read a sanitizer/miri diagnostic - see
// CONFIRMATION.md section 5) that captured the literal NaN values this exact call produces, and
// this candidate folder's `diagnostic_before.patch` / `diagnostic_after.patch` /
// `justfile`'s `diagnose` recipe to reproduce that capture yourself against a fresh clone.

use kurbo::{CurveFitSample, ParamCurveFit, Point, Vec2, fit_to_cubic};
use std::ops::Range;

struct StationarySpicy;

impl ParamCurveFit for StationarySpicy {
    fn sample_pt_tangent(&self, t: f64, _sign: f64) -> CurveFitSample {
        let p = Point::new(10.0 * t, 0.0);
        // Tangent direction spins rapidly - unrelated to the (zero) derivative reported by
        // sample_pt_deriv below. This is what flags the range as "spicy".
        let theta = t * 40.0;
        let tangent = Vec2::new(theta.cos(), theta.sin());
        CurveFitSample { p, tangent }
    }

    fn sample_pt_deriv(&self, t: f64) -> (Point, Vec2) {
        // Same position as sample_pt_tangent (so `moment_integrals`'s own zero-area result is
        // legitimately correct for this genuinely-straight source, not itself corrupted) - but a
        // genuinely zero derivative everywhere, which is what triggers the bug.
        (Point::new(10.0 * t, 0.0), Vec2::ZERO)
    }

    fn break_cusp(&self, _range: Range<f64>) -> Option<f64> {
        None
    }
}

fn main() {
    let source = StationarySpicy;
    let accuracy = 0.1;
    let range = 0.0..1.0;

    let result = fit_to_cubic(&source, range.clone(), accuracy);
    match &result {
        Some((c, err2)) => println!("fit_to_cubic({range:?}) -> Some(curve={c:?}, err2={err2})"),
        None => println!("fit_to_cubic({range:?}) -> None"),
    }

    println!();
    println!(
        "This call deterministically reaches CurveDist::compute_arc_params with `deriv.hypot()`"
    );
    println!(
        "== 0.0 at all 110 sub-samples, making the internal `arclen` sum exactly 0.0 and"
    );
    println!(
        "`arclen.recip()` == +inf - see RESULT.md / output.txt for the captured internal NaN this"
    );
    println!("produces in `arcparams`, and `just diagnose` in this folder to reproduce it.");
}
