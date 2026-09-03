// Reproduces QUADBEZ_ARCLEN_DEGENERATE_POINT_NAN: `QuadBez::arclen`'s analytical closed-form
// branch (`impl ParamCurveArclen for QuadBez`, kurbo/src/quadbez.rs) is taken instead of the
// near-straight quadrature fallback when a `QuadBez`'s three control points all coincide
// (p0 == p1 == p2), because the guard `a < 5e-4 * c` evaluates `0.0 < 0.0` (false) in that case.
// Inside the closed-form branch, `a2 = a.powf(-0.5)` becomes `+inf` (since `a == 0.0`), and
// `b = 2.0 * d2.dot(d1)` is exactly `0.0`, so `b * a2` evaluates `0.0 * inf = NaN`, which poisons
// the returned arc length. This uses only kurbo's public API (`QuadBez::arclen`, `BezPath`,
// `Shape::perimeter`/`Shape::area`, `QuadSpline::to_quads`) end-to-end - no internal fields are
// poked, and a fully degenerate quadratic segment (all three control points equal) is a real
// artifact a caller can hand in (e.g. a font/SVG outline with a repeated point collapsing a
// segment, or `QuadSpline::to_quads` over a spline with a duplicated knot - reproduced below too).

use kurbo::{BezPath, ParamCurveArclen, Point, QuadBez, QuadSpline, Shape};

fn check(label: &str, value: f64, saw_nan: &mut bool) {
    println!("{label} = {value}");
    if value.is_nan() {
        *saw_nan = true;
    }
}

fn main() {
    let mut saw_nan = false;

    // The exact case from the candidate report: p0 == p1 == p2, a fully degenerate QuadBez.
    let q = QuadBez::new(Point::new(3.0, 4.0), Point::new(3.0, 4.0), Point::new(3.0, 4.0));
    check("degenerate QuadBez::arclen(1e-6)", q.arclen(1e-6), &mut saw_nan);

    // Same degeneracy at the origin, and at a negative-coordinate point, to vary the input
    // within the same bug class (not just the one triggering value).
    let q_origin = QuadBez::new(Point::new(0.0, 0.0), Point::new(0.0, 0.0), Point::new(0.0, 0.0));
    check("origin-degenerate arclen", q_origin.arclen(1e-9), &mut saw_nan);

    let q_neg = QuadBez::new(
        Point::new(-7.5, -2.25),
        Point::new(-7.5, -2.25),
        Point::new(-7.5, -2.25),
    );
    check("negative-degenerate arclen", q_neg.arclen(1e-9), &mut saw_nan);

    // Poisoning the public Shape::perimeter entry point: a BezPath containing one degenerate
    // QuadTo segment (otherwise a normal rectangle-like path).
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.line_to((10.0, 0.0));
    path.quad_to((10.0, 0.0), (10.0, 0.0)); // degenerate: control point == both endpoints
    path.line_to((10.0, 10.0));
    path.line_to((0.0, 10.0));
    path.close_path();
    check("BezPath::perimeter with one degenerate QuadTo", path.perimeter(1e-6), &mut saw_nan);
    println!("BezPath::area (should stay correct/finite) = {}", path.area());

    // A realistic trigger: QuadSpline::to_quads over a spline with a duplicated knot, which
    // collapses one of the produced quads to a degenerate (all-coincident-point) segment.
    let pts = vec![
        Point::new(0.0, 0.0),
        Point::new(5.0, 5.0),
        Point::new(5.0, 5.0),
        Point::new(5.0, 5.0),
        Point::new(10.0, 0.0),
    ];
    let spline = QuadSpline::new(pts);
    for (i, quad) in spline.to_quads().enumerate() {
        check(&format!("QuadSpline::to_quads() quad {i} arclen"), quad.arclen(1e-9), &mut saw_nan);
    }

    if saw_nan {
        eprintln!("REPRODUCED - NaN observed from QuadBez::arclen on a degenerate segment");
        std::process::exit(1);
    } else {
        println!("did not reproduce - no NaN in any of the above");
    }
}
