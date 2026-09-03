// Standalone reproduction for issue #426: kurbo::flatten() incorrectly flattens a flat cubic
// Bezier that contains a cusp, silently dropping the whole interior of the curve instead of
// producing a polyline that actually follows it.
//
// Input (from the issue, "flat" meaning y is constant along the whole curve):
//   M 0 10 C -10 10 180 10 60 10
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use kurbo::{flatten, BezPath, CubicBez, ParamCurve, PathEl};

fn main() {
    let mut path = BezPath::new();
    path.move_to((0.0, 10.0));
    path.curve_to((-10.0, 10.0), (180.0, 10.0), (60.0, 10.0));

    let cubic = CubicBez::new((0.0, 10.0), (-10.0, 10.0), (180.0, 10.0), (60.0, 10.0));

    // Ground truth: the real cubic's maximum x, found by brute-force sampling of the same
    // public `eval` API `flatten` is supposed to approximate. The curve is degenerate (y is
    // constant everywhere) but still has real extent in x, peaking around x ~= 99.88 before
    // curving back down to the endpoint at x = 60.
    let mut reference_max_x = f64::NEG_INFINITY;
    for i in 0..=100_000 {
        let t = i as f64 / 100_000.0;
        reference_max_x = reference_max_x.max(cubic.eval(t).x);
    }

    let tolerance = 0.1;
    let mut flattened_max_x = f64::NEG_INFINITY;
    let mut num_points = 0usize;
    flatten(path.clone(), tolerance, |el| {
        num_points += 1;
        if let PathEl::MoveTo(p) | PathEl::LineTo(p) = el {
            flattened_max_x = flattened_max_x.max(p.x);
        }
    });

    println!("input path: {path:?}");
    println!("reference (brute-force sampled) max x on the real curve: {reference_max_x:.6}");
    println!("flatten() produced {num_points} path elements, max x: {flattened_max_x:.6}");

    // A correct flattening at tolerance 0.1 must reach within a small margin of the real
    // curve's extent. The bug collapses the whole cubic to a straight line from p0 (x=0) to p3
    // (x=60), so flattened_max_x stays at 60.0 - roughly 40 units short of the real peak.
    let deviation = reference_max_x - flattened_max_x;
    if deviation > 1.0 {
        eprintln!(
            "REPRODUCED - flatten() missed the cusp: real curve reaches x={reference_max_x:.6}, \
             but the flattened polyline only reaches x={flattened_max_x:.6} (deviation {deviation:.6}, \
             tolerance was {tolerance})"
        );
        std::process::exit(1);
    }

    println!("did not reproduce - flatten() output tracks the real curve within tolerance");
}
