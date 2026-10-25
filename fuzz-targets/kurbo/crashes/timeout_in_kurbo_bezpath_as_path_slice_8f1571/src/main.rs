use kurbo::Arc;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::as_path_slice(&kurbo::expand_path_signed(Arc { center: Point::from((0.0f64, 0.0f64)), radii: Vec2 { x: 0.0f64, y: 0.0f64 }, start_angle: 0.0f64, sweep_angle: -1.1805916207174112e21f64, x_rotation: 0.0f64 }, Diagonal2::new(0.0f64, 0.0f64), Join::Miter, 0.0f64, 0.0f64));
}
