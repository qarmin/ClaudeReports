// Reproduces PATHSEG_INTERSECT_LINE_DEGENERATE_PROBE_NAN: `PathSeg::intersect_line` (public API)
// computes `invlen2 = (dx*dx + dy*dy).recip()` for the probe `Line`'s direction, unguarded, in
// the `Quad`/`Cubic` match arms. A near-zero-length probe line (`dx == dy == 0.0`, e.g. a `Line`
// built from two equal/near-equal points) makes `invlen2` blow up toward `f64::INFINITY`. The
// numerator `(x - p0.x) * dx + (y - p0.y) * dy` is then always exactly `0.0` (since dx == dy ==
// 0.0), so `u = 0.0 * inf == NaN`. `(0.0..=1.0).contains(&NaN)` is always `false` (NaN compares
// false against everything), so a real, on-curve intersection point is silently dropped instead
// of being reported (or erring) - this uses only kurbo's public API (`QuadBez`, `CubicBez`,
// `PathSeg`, `Line`, `ParamCurve::eval`) end-to-end, no internal fields are poked.

use kurbo::{CubicBez, Line, ParamCurve, PathSeg, Point, QuadBez};

fn main() {
    let mut reproduced = false;

    // Quad case: probe a point that is genuinely on the curve at t = 0.5 with a truly
    // zero-length probe Line (both endpoints identical).
    let q = QuadBez::new(
        Point::new(0.0, 0.0),
        Point::new(50.0, 100.0),
        Point::new(100.0, 0.0),
    );
    let seg = PathSeg::Quad(q);
    let p = q.eval(0.5);
    let hits = seg.intersect_line(Line::new(p, p));
    println!("Quad: point {p:?} is exactly on curve at t=0.5; intersect_line(Line::new(p, p)) -> {hits:?}");
    if hits.is_empty() {
        eprintln!("REPRODUCED (Quad): genuine on-curve intersection silently dropped");
        reproduced = true;
    }

    // Same, but with a merely near-zero-length probe line (dx = 1e-100, not exactly zero) -
    // shows the false negative persists well beyond the exactly-degenerate case.
    let p2 = q.eval(0.3);
    let hits2 = seg.intersect_line(Line::new(p2, Point::new(p2.x + 1e-100, p2.y)));
    println!(
        "Quad: point {p2:?} on curve at t=0.3; near-zero-length probe (dx=1e-100) -> {hits2:?}"
    );
    if hits2.is_empty() {
        eprintln!("REPRODUCED (Quad, near-zero probe): genuine on-curve intersection silently dropped");
        reproduced = true;
    }

    // Cubic case: same shape of bug in the PathSeg::Cubic arm.
    let c = CubicBez::new(
        Point::new(0.0, 0.0),
        Point::new(30.0, 90.0),
        Point::new(70.0, 90.0),
        Point::new(100.0, 0.0),
    );
    let segc = PathSeg::Cubic(c);
    let pc = c.eval(0.5);
    let hitsc = segc.intersect_line(Line::new(pc, pc));
    println!("Cubic: point {pc:?} is exactly on curve at t=0.5; intersect_line(Line::new(pc, pc)) -> {hitsc:?}");
    if hitsc.is_empty() {
        eprintln!("REPRODUCED (Cubic): genuine on-curve intersection silently dropped");
        reproduced = true;
    }

    if reproduced {
        std::process::exit(1);
    } else {
        println!("did not reproduce - all genuine on-curve degenerate-probe intersections were found");
    }
}
