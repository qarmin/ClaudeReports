use kurbo::PathEl;
use kurbo::Point;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let point = Point { x: 0.0f64, y: 0.0f64 };
    let path_el = PathEl::QuadTo(point.expand(), point);
    let _ = kurbo::fit_to_cubic(&SimplifyBezPath::new(vec![path_el]), 85.0f64..0.0f64, 0.0f64);
}
