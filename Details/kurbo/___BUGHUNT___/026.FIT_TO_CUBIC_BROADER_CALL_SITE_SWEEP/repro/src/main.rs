// Standalone reproduction for candidate 026: broader sweep of `fit_to_cubic`'s call sites and
// its shared control-arm-length safety valve, beyond the two already-fixed defects (candidate
// 008's build-loop `unwrap` panic, candidate 009's extreme wild-control-point case).
//
// This shows that candidate 009's own chosen mitigation (an absolute cap of `MAX_ARM_LEN = 10.0`,
// in units of chord length, on the control-arm lengths `cubic_fit` returns) is not tight enough:
// an ordinary, non-adversarial input - a 4-vertex polyline with a single sharp corner, not a
// crafted extreme case - already produces a visibly wrong ("wild") control point with an arm
// length of about 7.04, comfortably under that cap, via the exact same underlying mechanism
// (`CurveDist::eval_ray`'s one-sided nearest-ray-intersection check can be fooled by a candidate
// that swings far from the chord and back while still crossing every sample's ray near the
// correct point).
//
// Uses only kurbo's public API: `simplify_bezpath`, `SimplifyOptions`, `BezPath`.
//
// Note: this reproduces on vanilla kurbo (no candidate 008/009 patches applied at all) - it is
// an independent manifestation of the same class of defect, not dependent on either of those
// two fixes being present or absent.
//
// Exit code convention: 0 = did not reproduce (every coordinate stayed within a sane distance
// of the input's bounding box), non-zero = reproduced (a control point escaped the bounding
// box by a wide margin).

use kurbo::simplify::{SimplifyOptLevel, SimplifyOptions, simplify_bezpath};
use kurbo::{BezPath, PathEl, Point};

/// A plain 4-vertex polyline with one sharp corner (roughly a 175-degree turn at the second
/// vertex). Extracted (and reduced from 10 to 4 vertices) from an otherwise unremarkable random
/// polyline found via a short property-fuzz sweep of `simplify_bezpath`.
const PTS: &[(f64, f64)] = &[
    (365.01378096433245, 929.6901693507418),
    (-884.6077114288375, -40.33291308287755),
    (112.97096062302103, 701.902611506651),
    (-43.36661508921134, 585.5715807866359),
];

fn main() {
    let mut path = BezPath::new();
    path.move_to(Point::new(PTS[0].0, PTS[0].1));
    for (x, y) in &PTS[1..] {
        path.line_to(Point::new(*x, *y));
    }

    // Input bounding box (computed from PTS directly, not from kurbo, so this bound doesn't
    // depend on the code under test). For a plain polyline (no curvature at all in the source),
    // this is exactly the convex hull's own bounding box - a correct fit's control points
    // should stay close to it, not thousands of units away.
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    for (x, y) in PTS {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    println!("input bbox: x in [{min_x}, {max_x}], y in [{min_y}, {max_y}]");

    let options = SimplifyOptions::default().opt_level(SimplifyOptLevel::Optimize);
    let out = simplify_bezpath(path.iter(), 2.0, &options);
    println!("output: {}", out.to_svg());

    // Generous slack (50 units - the local chord lengths here are ~1100-1500 units) beyond the
    // input's own bounding box.
    const SLACK: f64 = 50.0;
    let (lo_x, hi_x) = (min_x - SLACK, max_x + SLACK);
    let (lo_y, hi_y) = (min_y - SLACK, max_y + SLACK);

    let mut worst: f64 = 0.0;
    for el in out.elements() {
        let pts: &[Point] = match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => std::slice::from_ref(p),
            PathEl::QuadTo(p1, p2) => {
                check_point(*p1, lo_x, hi_x, lo_y, hi_y, &mut worst);
                std::slice::from_ref(p2)
            }
            PathEl::CurveTo(p1, p2, p3) => {
                check_point(*p1, lo_x, hi_x, lo_y, hi_y, &mut worst);
                check_point(*p2, lo_x, hi_x, lo_y, hi_y, &mut worst);
                std::slice::from_ref(p3)
            }
            PathEl::ClosePath => &[],
        };
        for p in pts {
            check_point(*p, lo_x, hi_x, lo_y, hi_y, &mut worst);
        }
    }

    if worst > 0.0 {
        eprintln!(
            "REPRODUCED - a control/end point escaped the input bounding box (+/-{SLACK} slack) by {worst} units"
        );
        std::process::exit(1);
    } else {
        println!("did not reproduce - every point stayed within the input bounding box");
    }
}

fn check_point(p: Point, lo_x: f64, hi_x: f64, lo_y: f64, hi_y: f64, worst: &mut f64) {
    let over = (lo_x - p.x).max(p.x - hi_x).max(lo_y - p.y).max(p.y - hi_y);
    if over > *worst {
        *worst = over;
    }
}
