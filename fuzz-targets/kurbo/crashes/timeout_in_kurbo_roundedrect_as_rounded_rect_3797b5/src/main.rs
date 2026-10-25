use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let circle_segment = CircleSegment::new(Point::from((0.0f32, 0.0f32)), 1.0f64, 0.0f64, -1e300f64, 0.714f64);
    let arc = circle_segment.outer_arc();
    let _ = <kurbo::RoundedRect as kurbo::Shape>::as_rounded_rect(&<kurbo::BezPath as kurbo::Shape>::as_rounded_rect(&<kurbo::Arc as kurbo::Shape>::into_path(arc, 0.0f64)).unwrap());
}
