// Standalone reproduction: `CubicBez::tangents_to_point` (kurbo's public API) reports a
// false-positive "tangent point" - a t value where the curve's tangent line does NOT actually
// pass through the query point - for a cubic whose `tangents_to_point` quartic-coefficient
// computation produces a leading coefficient (c4) that is floating-point cancellation noise
// (mathematically the tangent-count problem degenerates to a cubic, i.e. c4 "should" be exactly
// zero) but is not exactly representable as such.
//
// This is the same underlying `common::solve_cubic` weakness confirmed in candidate
// 014.CUBIC_LINE_INTERSECT_FALSE_POSITIVE_ISSUE411 (PathSeg::intersect_line, PR #593's
// Shape::winding case), now shown to also be reachable through `solve_quartic`'s own
// `c0 == 0.0` exact-delegation branch: `solve_quartic` calls
// `solve_cubic(c1, c2, c3, c4)` directly (no magnitude-relative guard) whenever the quartic's
// own c0 coefficient happens to be exactly `0.0` - which occurs naturally in
// `tangents_to_point` whenever the query point equals the curve's own start point `p0`
// (a very ordinary case: e.g. asking for tangent lines from a point sampled from the curve
// itself).
//
// Uses only kurbo's public API (`CubicBez`, `ParamCurve`, `ParamCurveDeriv`) - no internal
// scaffolding, no hand-constructed "peer" input (this is a single-participant finding: a
// caller passing valid, finite, in-range f64 control points and a valid query point to a
// documented public entry point).
//
// Exit code convention: 0 = did not reproduce (no spurious tangent point reported - the goal
// after a fix), non-zero = reproduced (a t value was reported whose tangent line is nowhere
// near passing through the query point).

use kurbo::{CubicBez, ParamCurve, ParamCurveDeriv, Point};

fn check(curve: CubicBez, label: &str) -> bool {
    let p = curve.p0;
    let ts = curve.tangents_to_point(p);
    println!("{label}: tangents_to_point({:?}) = {:?}", p, ts);

    let mut reproduced = false;
    for t in &ts {
        if *t <= 1e-9 {
            // t == 0 (p itself) is a trivial, always-correct root - not interesting.
            continue;
        }
        let pt = curve.eval(*t);
        let deriv = curve.deriv().eval(*t).to_vec2();
        let to_p = p - pt;
        let denom = deriv.hypot() * to_p.hypot();
        let sin_angle = if denom > 0.0 {
            (deriv.cross(to_p) / denom).abs()
        } else {
            0.0
        };
        println!(
            "  t={:.12} sin(angle between tangent and line-to-query-point)={:e} (should be ~0 for a genuine tangent point)",
            t, sin_angle
        );
        // A genuine tangent point has sin(angle) ~ 0 (tangent line direction and
        // curve-point-to-query-point direction are parallel). sin(angle) close to 1 means
        // the reported t is nowhere near tangent - a spurious root.
        if sin_angle > 0.5 {
            reproduced = true;
        }
    }
    reproduced
}

fn main() {
    // Primary repro case: found via a directed search for cubics where the two
    // tangent-count-quartic "leading" cross-product terms (a, b, in `parameters()`'s sense)
    // are nearly parallel, with the query point set to the curve's own start point p0 (so
    // solve_quartic's c0 == 0.0 exactly).
    let curve1 = CubicBez::new(
        Point::new(0.0, 0.0),
        Point::new(-0.43637135755259404, 0.13402559792320617),
        Point::new(2.5517910942929625, 0.3054485920859401),
        Point::new(11.955545724138252, 0.5469326386854837),
    );

    // Second, independently-found case (different control points) confirming this is not a
    // one-off input-specific quirk - the same class of near-degenerate configuration
    // reproduces reliably.
    let curve2 = CubicBez::new(
        Point::new(0.0, 0.0),
        Point::new(-0.5432963701108896, 0.02710058536491052),
        Point::new(7.786031641088047, -0.9874264434768539),
        Point::new(33.32119933972512, -4.021883199288864),
    );

    let r1 = check(curve1, "curve1");
    let r2 = check(curve2, "curve2");

    if r1 || r2 {
        eprintln!(
            "REPRODUCED - tangents_to_point returned at least one spurious t value whose \
             tangent line does not actually pass through the query point"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - no spurious tangent point reported, as expected");
    }
}
