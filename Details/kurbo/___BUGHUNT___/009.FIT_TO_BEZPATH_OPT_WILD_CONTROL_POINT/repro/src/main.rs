// Standalone reproduction for candidate 009 (issue #604): `simplify_bezpath` at
// `SimplifyOptLevel::Optimize` can emit a cubic control point tens of thousands of units
// outside a small input's bounding box, while its own error estimator reports the fit as
// effectively exact (err^2 ~ 2e-20).
//
// Uses only kurbo's public API: `simplify_bezpath`, `SimplifyOptions`, `BezPath`.
//
// Exit code convention: 0 = did not reproduce (every coordinate stayed within a sane distance
// of the input's bounding box), non-zero = reproduced (a control point escaped the bounding
// box by a wide margin).

use kurbo::simplify::{SimplifyOptLevel, SimplifyOptions, simplify_bezpath};
use kurbo::{BezPath, PathEl, Point};

/// A shallow curve arriving at (493.93, 324), then a straight 78-unit
/// backtrack along y = 324. Taken verbatim from issue #604.
const PTS: &[(f64, f64)] = &[
    (486.53000000000003, 325.49),
    (488.93, 324.68),
    (491.41, 324.18),
    (493.93, 324.0),
    (415.74, 324.0),
];

fn main() {
    let mut path = BezPath::new();
    path.move_to(Point::new(PTS[0].0, PTS[0].1));
    for (x, y) in &PTS[1..] {
        path.line_to(Point::new(*x, *y));
    }

    // Input bounding box (computed from PTS directly, not from kurbo, so this bound doesn't
    // depend on the code under test).
    let (mut min_x, mut max_x, mut min_y, mut max_y) =
        (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
    for (x, y) in PTS {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    println!("input bbox: x in [{min_x}, {max_x}], y in [{min_y}, {max_y}]");

    let options = SimplifyOptions::default()
        .angle_thresh(0.25)
        .opt_level(SimplifyOptLevel::Optimize);
    let out = simplify_bezpath(path.iter(), 2.0, &options);
    println!("output: {}", out.to_svg());

    // Allow generous slack (10 units) beyond the input's own bounding box - a correct fit's
    // control points should stay close to the geometry they approximate, not thousands of
    // units away.
    const SLACK: f64 = 10.0;
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
