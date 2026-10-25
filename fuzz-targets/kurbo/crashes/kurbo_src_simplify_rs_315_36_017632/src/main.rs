use kurbo::PathEl;
use kurbo::Point;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let _ = kurbo::simplify::simplify_bezpath(vec![PathEl::LineTo(Point::from((0.0f32, 0.0f32)))], 0.0f64, &SimplifyOptions::default());
}
