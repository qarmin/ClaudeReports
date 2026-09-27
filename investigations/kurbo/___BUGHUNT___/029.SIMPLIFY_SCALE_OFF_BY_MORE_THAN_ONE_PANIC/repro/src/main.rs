// Standalone reproduction for candidate 029: SimplifyBezPath::scale() maps t via
// `t * n` with no clamping of `t` to [0, 1], and both trait-method call sites that
// consume the result (`sample_pt_deriv` / `sample_pt_tangent` in kurbo/src/simplify.rs)
// only special-case `i == n` (t exactly at the end), not `i > n` (t further out of
// range). This calls the real, fully public API exactly as an external crate would:
// `simplify::SimplifyBezPath` is a `pub struct` in a `pub mod`, and `ParamCurveFit`
// (which declares `sample_pt_deriv`/`sample_pt_tangent`) is re-exported at the crate
// root.
//
// Exit code convention: 0 = did not reproduce, non-zero (via a caught panic) = reproduced.

use kurbo::simplify::SimplifyBezPath;
use kurbo::{BezPath, ParamCurveFit, Point};

fn main() {
    // A 2-segment path (SimplifyBezPath internally normalizes both segments to cubics,
    // so `n = self.0.len() == 2`).
    let mut path = BezPath::new();
    path.move_to(Point::new(0.0, 0.0));
    path.line_to(Point::new(10.0, 0.0));
    path.line_to(Point::new(10.0, 10.0));

    let s = SimplifyBezPath::new(path);

    // t = 1.6: t_scale = 1.6 * 2 = 3.2, i = floor(3.2) = 3, which is out of range for a
    // 2-element Vec (valid indices 0..=1). `i == n` (n == 2) is false, so the existing
    // clamp-back-into-range special case never triggers.
    let t = 1.6_f64;

    println!("SimplifyBezPath with n=2 segments; calling sample_pt_deriv({t}) ...");
    let deriv_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| s.sample_pt_deriv(t)));

    println!("calling sample_pt_tangent({t}, 1.0) ...");
    let tangent_result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| s.sample_pt_tangent(t, 1.0)));

    let mut reproduced = false;
    match deriv_result {
        Ok((p, d)) => println!("sample_pt_deriv({t}) did NOT panic: point={p:?} deriv={d:?}"),
        Err(_) => {
            println!("sample_pt_deriv({t}) PANICKED (reproduced)");
            reproduced = true;
        }
    }
    match tangent_result {
        Ok(sample) => println!(
            "sample_pt_tangent({t}, 1.0) did NOT panic: p={:?} tangent={:?}",
            sample.p, sample.tangent
        ),
        Err(_) => {
            println!("sample_pt_tangent({t}, 1.0) PANICKED (reproduced)");
            reproduced = true;
        }
    }

    if reproduced {
        eprintln!("REPRODUCED - see panic message(s) above");
        std::process::exit(1);
    } else {
        println!("did not reproduce - both calls completed normally");
    }
}
