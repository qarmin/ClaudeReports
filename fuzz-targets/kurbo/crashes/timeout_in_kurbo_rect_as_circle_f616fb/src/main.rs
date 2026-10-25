use kurbo::Arc;
use kurbo::Axis;
use kurbo::Point;

fn main() {
    let arc = Arc::new(Point::from((0.0f32, 0.0f32)), Axis::Vertical.pack_vec2(0.0f64, 0.0f64), 0.0f64, -6.067230110266265e29f64, 0.0f64);
    let _ = <kurbo::Rect as kurbo::Shape>::as_circle(&<kurbo::Arc as kurbo::Shape>::bounding_box(&arc));
}
