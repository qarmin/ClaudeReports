// Standalone reproduction of CUBIC_OFFSET_DEGENERATE_TANGENT_ATAN2.
//
// Root cause: `PathSeg::tangents()` (kurbo/src/bezpath.rs), which `CubicOffset::offset_cubic`
// (kurbo/src/offset.rs) uses to compute an endpoint tangent robustly, decides whether a
// difference vector like `p1 - p0` is "real" or "degenerate" (and should fall back to `p2 - p0`)
// with an ABSOLUTE threshold: `d01.hypot2() > 1e-12`. At large coordinate magnitudes (e.g.
// 1e10), a single f64 ULP of separation between p0 and p1 already has a squared length above
// 1e-12 - so a control point that is only floating-point-rounding-noise away from its neighbor
// is accepted as a real, meaningful tangent DIRECTION instead of triggering the fallback. The
// resulting unit tangent points in a essentially arbitrary direction (whichever axis the noise
// happened to land on), not the curve's real initial direction - which then feeds directly into
// `offset_cubic`'s very first control point placement (`c.p0 + d * utan0.turn_90()`), silently
// producing a geometrically wrong offset curve at the start of the segment.
//
// This runs the REAL public end-to-end entry point (`kurbo::stroke()`), not `offset_cubic`
// directly, and not any hand-constructed internal state.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced.

use kurbo::{BezPath, CubicBez, Point, Stroke, StrokeOpts, Vec2};

/// Stroke a single cubic segment and return the vector from its start point to the first point
/// of the resulting stroke outline (i.e. the observed initial offset direction, unnormalized).
fn stroked_start_offset(c: CubicBez, width: f64) -> Vec2 {
    let mut path = BezPath::new();
    path.move_to(c.p0);
    path.curve_to(c.p1, c.p2, c.p3);
    let stroked = kurbo::stroke(&path, &Stroke::new(width), &StrokeOpts::default(), 1e-3);
    let first = stroked
        .iter()
        .next()
        .expect("stroke should produce at least a MoveTo");
    match first {
        kurbo::PathEl::MoveTo(p) => p - c.p0,
        other => panic!("expected MoveTo as first element, got {other:?}"),
    }
}

fn angle_between(a: Vec2, b: Vec2) -> f64 {
    a.normalize().cross(b.normalize()).atan2(a.normalize().dot(b.normalize()))
}

fn main() {
    let base = 1e10_f64;
    let ulp = base.next_up() - base;
    println!("base = {base}, ulp at this magnitude = {ulp:e}");

    let p0 = Point::new(base, base);
    // Real intended curve direction/shape: p1 nudged only 1 ULP in y away from p0 - a
    // floating-point-noise-scale perturbation, unrelated to the curve's actual shape.
    let p1_noise = Point::new(base, base + ulp);
    let p2 = Point::new(base + 50.0, base + 200.0);
    let p3 = Point::new(base + 300.0, base);

    let c_noise = CubicBez::new(p0, p1_noise, p2, p3);

    // Reference/oracle curve: p1 == p0 exactly. This is the FULLY degenerate case, which
    // `PathSeg::tangents()` has always handled correctly (on both patched and unpatched trees)
    // via its explicit fallback chain (falls through to `p2 - p0`). It represents the same
    // intended curve shape/direction as `c_noise` above, just without the spurious few-ULP
    // perturbation - so its stroked start direction is the ground truth `c_noise` should match.
    let c_reference = CubicBez::new(p0, p0, p2, p3);

    let width = 10.0;
    let offset_noise = stroked_start_offset(c_noise, width);
    let offset_reference = stroked_start_offset(c_reference, width);

    println!("d(p1,p0) hypot2 for the noise curve = {:e}", (p1_noise - p0).hypot2());
    println!("stroked start offset (noise curve)     = {offset_noise:?}");
    println!("stroked start offset (reference curve) = {offset_reference:?}");

    let angle_deg = angle_between(offset_noise, offset_reference).to_degrees();
    println!("angle between the two start directions = {angle_deg:.3} degrees");

    // A 1-2 ULP perturbation is not a real geometric feature of the curve - the two directions
    // should be (numerically) identical. Anything beyond a couple of degrees means the ULP-scale
    // noise silently flipped the computed tangent to an unrelated direction.
    const MAX_EXPECTED_ANGLE_DEG: f64 = 2.0;
    if angle_deg.abs() > MAX_EXPECTED_ANGLE_DEG {
        eprintln!(
            "REPRODUCED: stroked start direction differs by {angle_deg:.3} degrees between a \
             curve perturbed by a single float ULP and its otherwise-identical fully-degenerate \
             reference, exceeding the {MAX_EXPECTED_ANGLE_DEG} degree tolerance."
        );
        std::process::exit(1);
    }
    println!("did not reproduce - both start directions agree within tolerance");
}
