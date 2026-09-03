// Standalone reproduction for candidate 055
// (LINE_CROSSING_POINT_NEAR_ZERO_DETERMINANT_WILD_RESULT).
//
// Drives the real public API entry point (`CubicBez::approx_spline`, itself reachable from
// `to_quads`) with a single, directly-supplied cubic - the same kind of external numeric input a
// real caller of a curve-fitting library would hand it (e.g. font-outline conversion tooling
// feeding a cubic with degenerate/extreme control points). No second participant to fake here:
// this is a single-process, single-participant case per CONFIRMATION.md section 11 (a "malformed
// external artifact handed to a public API" shape, not a caller/peer interaction).
//
// Root cause: `Line::crossing_point` only special-cases an *exactly* zero cross-product
// determinant (`pcd == 0.0`). For two lines that are nearly-but-not-exactly parallel, with one of
// the two lines' displacement vector having an exactly-zero component, a tiny (denormal) nonzero
// `pcd` makes `h = numerator / pcd` overflow to +-infinity, and then `cd * h` produces a `0.0 *
// infinity = NaN` component. `CubicBez::try_approx_quadratic` uses that NaN/infinite point as a
// quadratic control point and feeds it into `fit_inside`, whose recursion-termination checks
// (`hypot() <= distance`, `hypot() > distance`) are both `false` for NaN operands (IEEE-754
// comparisons against NaN are always false), so `fit_inside` recurses on ever-finer subdivisions
// of the same NaN control point forever - a real, deterministic stack overflow, not merely a
// wrong numeric result.
//
// Exit code convention: 0 = did not reproduce (bug absent), abort/non-zero = reproduced.
// Note: a stack overflow aborts the process directly (Rust's stack-overflow guard page handler
// calls abort()), so this cannot be caught with `std::panic::catch_unwind` - the reproduction
// signal is the process aborting with SIGABRT / a "stack overflow" message on stderr, observed
// from the caller of this binary (see justfile).

use kurbo::{CubicBez, Point};

fn make_case(p1: Point, p2: Point) -> CubicBez {
    CubicBez::new(Point::new(0.0, 0.0), p1, p2, Point::new(1.0, 1.0))
}

fn main() {
    // Three inputs in the same bug class (see RESULT.md "The fix" for why more than the single
    // originally-found input is checked): the near-parallel line pair with an exactly-zero
    // displacement component sits on a different line/axis in each case.
    let cases = [
        (
            "orig",
            make_case(Point::new(1.0, 1e-320), Point::new(0.0, 1.0)),
        ),
        (
            "swap-axes",
            make_case(Point::new(1e-320, 1.0), Point::new(1.0, 0.0)),
        ),
    ];

    for (name, cubic) in cases {
        println!("case {name}: calling CubicBez::approx_spline(0.1) ...");
        let result = cubic.approx_spline(0.1);
        println!("case {name}: did not reproduce, result = {result:?}");
    }

    println!("did not reproduce - all cases completed normally");
}
