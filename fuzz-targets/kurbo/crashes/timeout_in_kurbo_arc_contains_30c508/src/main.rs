use kurbo::Arc;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Arc as kurbo::Shape>::contains(&Arc { center: Point::default(), radii: Vec2::new(0.0f64, 0.0f64), start_angle: 0.0f64, sweep_angle: f64::MIN, x_rotation: 0.0f64 }, Point::ORIGIN);
}
