use kurbo::Arc;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::into_path(kurbo::expand_path(Arc::new(Point::ZERO, Vec2::from_angle(1.0f64), 0.0f64, 1.0f64, 0.0f64), Diagonal2::new(0.0f64, 0.0f64), Join::Bevel, 0.0f64, 0.0f64), 0.0f64);
}
