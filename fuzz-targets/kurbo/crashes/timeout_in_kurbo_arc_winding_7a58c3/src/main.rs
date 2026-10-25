use kurbo::Arc;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let point = Point::ZERO;
    let _ = <kurbo::Arc as kurbo::Shape>::winding(&Arc::new(point, Vec2::ZERO, 0.0f64, f64::INFINITY, 0.0f64), point);
}
