// Standalone reproduction for candidate 034 (BumpEstimator::count_path LineTo unwrap panic).
//
// Uses vello_encoding's public API (`BumpEstimator::count_path`) exactly as `vello::Scene::fill`/
// `stroke` would drive it internally when handed a hand-built `peniko::kurbo::BezPath` whose
// first element is a `LineTo` with no preceding `MoveTo` - a shape kurbo itself never rejects
// (`BezPath::from_vec` performs no such validation), so any real caller can construct one and
// pass it straight through vello's public shape-drawing API.
//
// Exit code convention: 0 = did not reproduce (bug absent), non-zero = reproduced (bug present).

use peniko::kurbo::{Affine, PathEl, Point};
use vello_encoding::{BumpEstimator, Transform};

fn main() {
    // The exact minimal triggering input: a single LineTo, no MoveTo before it.
    let path = vec![PathEl::LineTo(Point::new(5.0, 5.0))];

    let result = std::panic::catch_unwind(|| {
        let mut est = BumpEstimator::new();
        est.count_path(path.into_iter(), &Transform::from_kurbo(&Affine::IDENTITY), None);
    });

    match result {
        Ok(_) => {
            println!("did not reproduce - suspect code path completed normally");
        }
        Err(panic_payload) => {
            eprintln!("REPRODUCED - panicked: {:?}", panic_payload);
            std::process::exit(1);
        }
    }
}
