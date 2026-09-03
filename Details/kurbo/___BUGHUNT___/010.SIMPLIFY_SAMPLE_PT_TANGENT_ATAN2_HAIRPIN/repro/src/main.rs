// Checks whether `CubicOffset` (kurbo/src/offset.rs) shares the same
// zero-derivative-at-endpoint -> atan2(0,0) -> hairpin footgun that PR #606 fixes in
// `SimplifyBezPath::sample_pt_tangent` (kurbo/src/simplify.rs).
//
// The trigger shape is identical in both cases: a straight line represented as a cubic via
// `PathSeg::Line::to_cubic()`, which produces `CubicBez::new(p0, p0, p1, p1)` - control points
// coincident with the endpoints, so the raw derivative vanishes at t=0 and t=1.
//
// Two checks, both through kurbo's public API:
//   1. `kurbo::offset::offset_cubic` called directly on that exact degenerate cubic - this is
//      the "shared fitting code" entry point named in the candidate.
//   2. `kurbo::stroke()` (the real end-to-end public entry point a caller actually uses) fed a
//      path whose only segment is a `CurveTo` with those same degenerate control points, so the
//      real driver code decides whether/how to call into `offset_cubic`.
//
// Exit code convention: 0 = did not reproduce (no hairpin found in either check), non-zero =
// reproduced (a hairpin/reversed tangent was found).

use kurbo::{BezPath, CubicBez, ParamCurve, Point, Stroke};

fn check_offset_cubic_direct() -> bool {
    // Exactly what `PathSeg::Line { p0: (0,0), p1: (100,0) }.to_cubic()` produces: control
    // points collapsed onto the endpoints, so the raw derivative vanishes at t=0 and t=1 -
    // the same degenerate shape that broke `SimplifyBezPath::sample_pt_tangent`.
    let c = CubicBez::new(
        Point::new(0.0, 0.0),
        Point::new(0.0, 0.0),
        Point::new(100.0, 0.0),
        Point::new(100.0, 0.0),
    );
    let mut result = BezPath::new();
    kurbo::offset::offset_cubic(c, 5.0, 0.1, &mut result);

    // The exact correct answer is the straight line from (0, 5) to (100, 5) (offset +5 in y).
    // Check the actual endpoint the approximation lands on: a hairpin/reversed-direction bug
    // would leave the curve ending up back near its start (0, 5) instead, or produce
    // NaN/non-finite control points.
    let end = result
        .segments()
        .last()
        .expect("offset_cubic produced no segments")
        .end();
    println!("offset_cubic (direct): endpoint = ({:+.6}, {:+.6})", end.x, end.y);
    if !end.x.is_finite() || !end.y.is_finite() {
        println!("  -> non-finite endpoint - treating as reproduced/broken");
        return false;
    }
    let ok = (end.x - 100.0).abs() < 1.0 && (end.y - 5.0).abs() < 1.0;
    println!(
        "  -> {}",
        if ok {
            "OK (landed at the expected offset endpoint)"
        } else {
            "HAIRPIN/WRONG (did not land at the expected offset endpoint)"
        }
    );
    ok
}

fn check_stroke_end_to_end() -> bool {
    // A path whose single segment is a CurveTo with control points collapsed onto the
    // endpoints - i.e. a straight line expressed as a non-regular cubic, the same degenerate
    // shape `SimplifyBezPath` produces internally from `Line::to_cubic()`. This exercises the
    // real public `stroke()` entry point end-to-end, not the internal offset module directly.
    let mut path = BezPath::new();
    path.move_to((0.0, 0.0));
    path.curve_to((0.0, 0.0), (100.0, 0.0), (100.0, 0.0));

    let style = Stroke::new(10.0);
    let tolerance = 0.1;
    let opts = kurbo::StrokeOpts::default();
    let outline = kurbo::stroke(path.elements().iter().copied(), &style, &opts, tolerance);

    println!("stroke() outline elements:");
    for el in outline.elements() {
        println!("  {el:?}");
    }

    let segs: Vec<_> = outline.segments().collect();
    if segs.is_empty() {
        panic!("stroke() produced an empty outline");
    }
    // Find the outline segment terminating farthest in +x - that's the forward-side offset
    // curve that travelled alongside the source line's direction of travel. Check that its
    // arrival tangent isn't reversed (pointing back toward -x).
    let mut worst_ok = true;
    for seg in &segs {
        let (p2, p3) = match seg {
            kurbo::PathSeg::Cubic(c) => (c.p2, c.p3),
            kurbo::PathSeg::Line(l) => (l.p0, l.p1),
            kurbo::PathSeg::Quad(q) => (q.p1, q.p2),
        };
        let arrival = p3 - p2;
        if arrival.hypot() > 1e-9 && arrival.x.abs() > arrival.y.abs().max(1e-9) {
            // A segment whose arrival is dominantly along x: it must not be travelling -x if
            // it's part of the forward-direction offset (near y ~ -5) side.
            if p3.y < 0.0 && arrival.x < -1e-6 {
                println!(
                    "stroke() outline segment ending at {:?} arrives travelling ({:+.6}, {:+.6}) - HAIRPIN suspected",
                    p3, arrival.x, arrival.y
                );
                worst_ok = false;
            }
        }
    }
    println!(
        "stroke() end-to-end: outline has {} segments, hairpin_found={}",
        segs.len(),
        !worst_ok
    );
    worst_ok
}

// Separate, informational only: a *fully* degenerate cubic (all four control points equal, a
// zero-length curve) - `offset_cubic`'s own doc comment already disclaims this class of input
// ("not suitable for degenerate cubics with entirely co-linear control points... should be
// handled before calling this function"), and it is not the same failure pattern as the
// `SimplifyBezPath` hairpin bug (a *silent wrong direction*, not NaN). Recorded here only to
// show what actually happens, not counted toward the pass/fail verdict above.
fn check_fully_degenerate_point_cubic() {
    let p = Point::new(50.0, 50.0);
    let c = CubicBez::new(p, p, p, p);
    let mut result = BezPath::new();
    kurbo::offset::offset_cubic(c, 5.0, 0.1, &mut result);
    let n = result.elements().len();
    let end = result.segments().last().map(|s| s.end());
    println!(
        "\n(informational, not part of the verdict) fully-degenerate point cubic: {n} elements produced, last endpoint = {end:?}"
    );
    println!(
        "  -> this is the pre-existing, documented \"not suitable for degenerate cubics\" limitation, a different failure mode (NaN) than the hairpin pattern checked above, and not reachable via the public stroke() driver (do_cubic() routes such input to do_linear() first)."
    );
}

fn main() {
    let ok1 = check_offset_cubic_direct();
    let ok2 = check_stroke_end_to_end();
    check_fully_degenerate_point_cubic();

    if ok1 && ok2 {
        println!("\ndid not reproduce - CubicOffset does not show the atan2(0,0) hairpin pattern");
    } else {
        eprintln!("\nREPRODUCED - CubicOffset shows a hairpin for a degenerate endpoint tangent");
        std::process::exit(1);
    }
}
