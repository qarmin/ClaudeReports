use kurbo::Arc;
use kurbo::Axis;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Arc as kurbo::Shape>::to_path(&Arc { center: Axis::Horizontal.pack_point(0.0f64, 0.0f64), radii: Vec2::default(), start_angle: 0.0f64, sweep_angle: 1e300f64, x_rotation: 0.0f64 }, 0.0f64);
}
