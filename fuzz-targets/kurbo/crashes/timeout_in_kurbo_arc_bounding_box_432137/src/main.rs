use kurbo::Arc;
use kurbo::Axis;
use kurbo::Point;
use kurbo::SvgParseError;
use kurbo::Vec2;

fn main() {
    let axis = Axis::Vertical;
    let vec2 = axis.pack_vec2(6.193250200388348e-167f64, 0.546f64);
    let _ = <kurbo::Arc as kurbo::Shape>::bounding_box(&Arc { center: <kurbo::Point as ::core::ops::Add<kurbo::Vec2>>::add(Point::default(), vec2), radii: Vec2::ZERO, start_angle: -3.099657922243612e195f64, sweep_angle: 9007199254740992.0f64, x_rotation: -44.8f64 });
    let svg_parse_error = SvgParseError::UnexpectedEof;
    let _ = svg_parse_error.to_string();
}
