use kurbo::Arc;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Rect as kurbo::Shape>::perimeter(&<kurbo::Arc as kurbo::Shape>::bounding_box(&Arc::new(Point { x: 0.0f64, y: 0.0f64 }, Vec2 { x: f64::INFINITY, y: 0.0f64 }, 0.0f64, 1.0f64, 0.0f64)), 1.0f64);
}
