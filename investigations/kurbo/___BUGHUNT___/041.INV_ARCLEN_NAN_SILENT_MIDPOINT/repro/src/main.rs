// Standalone reproduction for candidate 041 (INV_ARCLEN_NAN_SILENT_MIDPOINT).
//
// Drives kurbo's real public entry point (`ParamCurveArclen::inv_arclen`, called through
// `CubicBez`) with curves that are each real, single-artifact geometric input - no second
// participant is being faked here (see CONFIRMATION.md section 11: a `CubicBez` handed straight
// to a public method is the genuine external input, same as a caller would construct).
//
// Root cause: `ParamCurveArclen::inv_arclen`'s default trait method (src/param_curve.rs) computes
// `total_arclen = self.arclen(accuracy)` and then guards `if arclen >= total_arclen { return 1.0 }`.
// When `total_arclen` is NaN (a curve with a non-finite/NaN control point, or fed by any other bug
// that makes `arclen()` return NaN), that comparison is always false for NaN operands, so the
// function falls through and computes `epsilon = accuracy / total_arclen` = NaN. That NaN
// `epsilon` is passed into `common::solve_itp` -> `solve_itp_fallible`, whose
// `n1_2 = (....).max(0.0)` uses `f64::max`, which by definition returns the *non-NaN* operand when
// one side is NaN - silently replacing the NaN with `0.0`. `scaled_epsilon` then also ends up NaN,
// so the ITP loop's own `while b - a > 2.0 * epsilon` condition is `1.0 > NaN` = false, the loop
// body never runs, and `solve_itp` returns the untouched initial bracket midpoint `0.5 * (0.0 +
// 1.0) = 0.5` - an arbitrary, meaningless parameter value - instead of propagating NaN or erroring.
//
// This repro exercises four cases through the same real public API call path:
//   1. A curve with an infinite (`f64::INFINITY`) control point (the case from the original
//      finding).
//   2. A curve with a NaN control point directly (a different NaN-producing input, same class).
//   3. A perfectly ordinary, finite curve, but a NaN `arclen` query argument.
//   4. A sanity check: an ordinary curve/query still returns a normal, finite `t` in [0, 1].
//
// Exit code convention: 0 = every case behaved correctly (NaN in -> NaN out, sane input -> sane
// output). Non-zero = at least one case silently returned a bogus finite value where NaN should
// have propagated - the real, currently-affected behavior on the pristine tree before
// `patch.diff`, and the expected clean run after it.

use kurbo::{CubicBez, ParamCurveArclen};

fn check_nan_propagates(name: &str, c: CubicBez, arclen_query: f64) -> bool {
    let total = c.arclen(1e-6);
    let t = c.inv_arclen(arclen_query, 1e-6);
    println!("[{name}] total_arclen={total} inv_arclen({arclen_query})={t}");
    if t.is_nan() {
        println!("[{name}] OK - NaN propagated as expected");
        true
    } else {
        println!("[{name}] REPRODUCED - silently returned bogus finite value {t} instead of NaN");
        false
    }
}

fn main() {
    let mut all_ok = true;

    // Case 1: infinite control point (the original finding).
    all_ok &= check_nan_propagates(
        "inf-ctrl-point",
        CubicBez::new((0.0, 0.0), (f64::INFINITY, 0.0), (2.0, 0.0), (3.0, 0.0)),
        1.0,
    );

    // Case 2: NaN control point directly - a different NaN-producing curve, same bug class.
    all_ok &= check_nan_propagates(
        "nan-ctrl-point",
        CubicBez::new((0.0, 0.0), (f64::NAN, 0.0), (2.0, 0.0), (3.0, 0.0)),
        1.0,
    );

    // Case 3: an ordinary, finite curve, but a NaN query argument.
    let normal = CubicBez::new((0.0, 0.0), (1.0, 1.0), (2.0, 1.0), (3.0, 0.0));
    all_ok &= check_nan_propagates("nan-query-arg", normal, f64::NAN);

    // Case 4 (sanity): an ordinary curve/query must still return a normal, finite t in [0, 1].
    let t = normal.inv_arclen(normal.arclen(1e-6) * 0.5, 1e-6);
    println!("[sanity-normal-curve] inv_arclen(half arclen) = {t}");
    let sane = t.is_finite() && (0.0..=1.0).contains(&t);
    if sane {
        println!("[sanity-normal-curve] OK - finite t in [0, 1]");
    } else {
        println!("[sanity-normal-curve] UNEXPECTED - the fix broke the normal, non-NaN case");
    }
    all_ok &= sane;

    if all_ok {
        println!("did not reproduce - all cases behaved correctly");
    } else {
        eprintln!("REPRODUCED overall - see cases marked REPRODUCED above");
        std::process::exit(1);
    }
}
