// Standalone reproduction for candidate 011: Line::eval, QuadBez::eval, and CubicBez::eval
// return NaN or an infinite coordinate at t=0/t=1 instead of the curve's own finite endpoint,
// when an intermediate control point has extreme magnitude relative to the endpoints.
//
// This is exactly the repro from kurbo issue #598, run through kurbo's real public API.

use kurbo::{CubicBez, Line, ParamCurve, Point, QuadBez};

fn check(name: &str, got: Point, expected: Point, failed: &mut bool) {
    let ok = got.x == expected.x && got.y == expected.y;
    println!(
        "{name}: got {got:?}, expected {expected:?} -> {}",
        if ok { "OK" } else { "MISMATCH" }
    );
    if !ok {
        *failed = true;
    }
}

fn main() {
    let mut failed = false;

    let l = Line::new(Point::new(1e308, 0.0), Point::new(-1e308, 0.0));
    check("Line eval(0)", l.eval(0.0), l.p0, &mut failed);
    check("Line eval(1)", l.eval(1.0), l.p1, &mut failed);

    let big = 1e308;
    let q = QuadBez::new(
        Point::new(0.0, 0.0),
        Point::new(big, big),
        Point::new(0.0, 0.0),
    );
    check("QuadBez eval(0)", q.eval(0.0), q.p0, &mut failed);
    check("QuadBez eval(1)", q.eval(1.0), q.p2, &mut failed);

    let c = CubicBez::new(
        Point::new(0.0, 0.0),
        Point::new(big, big),
        Point::new(0.0, 0.0),
        Point::new(0.0, 0.0),
    );
    check("CubicBez eval(0)", c.eval(0.0), c.p0, &mut failed);
    check("CubicBez eval(1)", c.eval(1.0), c.p3, &mut failed);

    if failed {
        eprintln!("REPRODUCED - eval() at an endpoint did not return the curve's own endpoint");
        std::process::exit(1);
    } else {
        println!("did not reproduce - all endpoint evals matched the curve's own endpoints");
    }
}
