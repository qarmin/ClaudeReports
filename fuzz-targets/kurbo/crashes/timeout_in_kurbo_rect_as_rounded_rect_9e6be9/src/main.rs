use kurbo::Arc;
use kurbo::Point;
use kurbo::Stroke;
use kurbo::Triangle;
use kurbo::Vec2;

fn main() {
    let _ = Stroke::default();
    let arc = <kurbo::Arc as kurbo::ParamCurve>::subsegment(&Arc { center: Point::new(49.0f64, 0.545f64), radii: Vec2::new(0.387f64, f64::INFINITY), start_angle: f64::INFINITY, sweep_angle: 195.0f64, x_rotation: 9007199254740992.0f64 }.reversed(), -0.0f64..255.0f64);
    let _ = Triangle::EQUILATERAL;
    let _ = <kurbo::Rect as kurbo::Shape>::as_rounded_rect(&<kurbo::Rect as kurbo::Shape>::bounding_box(&<kurbo::Arc as kurbo::Shape>::bounding_box(&arc).scale_from_origin(1e300f64)));
}
