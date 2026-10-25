use kurbo::Axis;
use kurbo::CircleSegment;

fn main() {
    let axis = Axis::Horizontal;
    let point = axis.pack_point(0.0f64, 0.0f64);
    let circle_segment = CircleSegment::new(point, 0.0f64, 0.0f64, 0.0f64, 1.3270698809889517e78f64);
    let arc = circle_segment.inner_arc();
    let _ = <kurbo::BezPath as kurbo::Shape>::to_path(&<kurbo::Arc as kurbo::Shape>::into_path(arc, 0.0f64), 0.0f64);
}
