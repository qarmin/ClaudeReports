use kurbo::CircleSegment;
use kurbo::Ellipse;
use kurbo::Point;

fn main() {
    let ellipse = Ellipse::default();
    let f64_v = ellipse.rotation();
    let _ = <kurbo::BezPath as kurbo::Shape>::winding(&<kurbo::Ellipse as kurbo::Shape>::into_path(ellipse, 1.0f64), <kurbo::Arc as kurbo::ParamCurve>::eval(&CircleSegment { center: Point::from((0.0f32, 0.0f32)), outer_radius: 0.0f64, inner_radius: f64::INFINITY, start_angle: f64_v, sweep_angle: 0.0f64 }.inner_arc(), 0.0f64));
}
