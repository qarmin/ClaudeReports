use kurbo::Arc;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let point = Point::new(0.0f64, 0.0f64);
    let vec2 = point.to_vec2();
    let _ = <kurbo::Rect as kurbo::Shape>::as_path_slice(&<kurbo::Arc as kurbo::Shape>::bounding_box(&Arc::new(Point::new(1.0f64, 1.0f64), Vec2::splat(f64::INFINITY).lerp(vec2, f64::MIN), 0.5f64, f64::MAX, 0.216f64)));
}
