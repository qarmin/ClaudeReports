// Standalone reproduction of a panic in `fit_to_bezpath_opt_inner` (kurbo/src/fit.rs).
//
// Uses kurbo's public API end-to-end: `SimplifyBezPath` (public `simplify` module) wraps a
// hand-built `BezPath` (an ordinary, valid, finite geometric input - not a stand-in for another
// participant's wire format), then the public `fit_to_bezpath_opt` entry point is called on it.
// This is the exact same input a real caller of `simplify_bezpath(..., SimplifyOptLevel::Optimize)`
// would produce internally, per GitHub issue #601.
//
// Exit code convention: 0 = did not reproduce, non-zero = reproduced (panicked).

use kurbo::simplify::SimplifyBezPath;
use kurbo::{BezPath, Point};

fn main() {
    // A closed quadrilateral whose four vertices are nearly collinear (issue #601's repro).
    let pts = [(1.0, 0.7), (4.1, 2.0), (4.4, 2.1), (0.0, 0.0)];

    let mut path = BezPath::new();
    path.move_to(Point::new(pts[0].0, pts[0].1));
    for (x, y) in &pts[1..] {
        path.line_to(Point::new(*x, *y));
    }
    path.close_path();

    let source = SimplifyBezPath::new(path);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        kurbo::fit_to_bezpath_opt(&source, 2.0)
    }));

    match result {
        Ok(_bezpath) => {
            println!("did not reproduce - fit_to_bezpath_opt completed normally");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
