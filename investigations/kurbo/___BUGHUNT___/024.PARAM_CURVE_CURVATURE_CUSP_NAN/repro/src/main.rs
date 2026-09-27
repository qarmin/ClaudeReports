// Reproduction for candidate 024: `ParamCurveCurvature::curvature`'s default trait-method
// implementation (kurbo/src/param_curve.rs) computes `d2.cross(d) * d.hypot2().powf(-1.5)` with
// no guard for a vanishing first derivative (a cusp). When the derivative is exactly zero,
// `d.hypot2().powf(-1.5)` is `0.0.powf(-1.5) == f64::INFINITY`, and `d2.cross(d)` (a cross
// product against the zero vector) is exactly `0.0`, so the product is `0.0 * INFINITY == NaN`.
//
// This is a straightforward, deterministic boundary-value bug in pure math on a public API -
// there is no second participant/peer/caller producing adversarial bytes to fake here (see
// CONFIRMATION.md section 11's carve-out): a `CubicBez` with `p0 == p1` (or `p2 == p3`) is a
// completely ordinary, directly-constructible value via kurbo's own public constructor.
//
// Exit code convention: 0 = did not reproduce (bug absent - the goal after a fix), non-zero =
// reproduced (bug present, i.e. curvature() returned NaN at a cusp).

use kurbo::{CubicBez, ParamCurveCurvature, Point};

fn main() {
    let mut reproduced = false;

    // Cusp at the start of the curve: p0 == p1, so the first derivative at t=0 is the zero vector.
    let p0 = Point::new(0.0, 0.0);
    let start_cusp = CubicBez::new(p0, p0, Point::new(1.0, 1.0), Point::new(1.0, 1.0));
    let k_start = start_cusp.curvature(0.0);
    println!("start-cusp curvature(0.0) = {k_start} (is_nan = {})", k_start.is_nan());
    if k_start.is_nan() {
        reproduced = true;
    }

    // Cusp at the end of the curve: p2 == p3, so the first derivative at t=1 is the zero vector.
    let p3 = Point::new(1.0, 1.0);
    let end_cusp = CubicBez::new(Point::new(0.0, 0.0), Point::new(0.5, 0.5), p3, p3);
    let k_end = end_cusp.curvature(1.0);
    println!("end-cusp curvature(1.0) = {k_end} (is_nan = {})", k_end.is_nan());
    if k_end.is_nan() {
        reproduced = true;
    }

    // Sanity check: an ordinary, non-degenerate curve must still produce a normal finite
    // curvature value both before and after any fix (guards against a fix on to catch this).
    let normal = CubicBez::new(
        Point::new(0.0, 0.0),
        Point::new(0.0, 1.0),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
    );
    let k_normal = normal.curvature(0.5);
    println!("normal-curve curvature(0.5) = {k_normal} (is_nan = {})", k_normal.is_nan());
    if k_normal.is_nan() || !k_normal.is_finite() {
        eprintln!("UNEXPECTED: a non-degenerate curve produced a non-finite curvature");
        std::process::exit(2);
    }

    if reproduced {
        eprintln!("REPRODUCED - curvature() returned NaN at a cusp (zero derivative)");
        std::process::exit(1);
    } else {
        println!("did not reproduce - curvature() at a cusp is finite");
    }
}
