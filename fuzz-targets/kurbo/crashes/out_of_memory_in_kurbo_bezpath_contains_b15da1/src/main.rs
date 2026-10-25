use kurbo::Arc;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::contains(&<kurbo::Arc as kurbo::Shape>::into_path(Arc { center: Point::from((0.0f32, 0.0f32)), radii: Vec2::from((0.0f64, 0.0f64)), start_angle: 2.2250738585072014e-308f64, sweep_angle: f64::INFINITY, x_rotation: -0.0f64 }, 0.0f64), Point::default());
}
