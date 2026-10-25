use kurbo::Axis;
use kurbo::CircleSegment;
use kurbo::LineIntersection;
use kurbo::SvgParseError;

fn main() {
    let _ = LineIntersection { line_t: 0.0f64, segment_t: 1.0f64 };
    let axis = Axis::Horizontal;
    let point = axis.pack_point(0.0f64, 0.0f64);
    let _ = SvgParseError::UnknownCommand('a');
    let _ = <kurbo::Line as kurbo::Shape>::winding(&<kurbo::Circle as kurbo::Shape>::as_line(&<kurbo::BezPath as kurbo::Shape>::as_circle(&<kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment::new(point, 0.0f64, 1.0f64, 0.0f64, 0.9999999999999999f64), 0.0f64)).unwrap()).unwrap(), point);
}
