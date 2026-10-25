use kurbo::Arc;
use kurbo::Axis;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Arc as kurbo::Shape>::perimeter(&Arc { center: Axis::Vertical.pack_vec2(0.0f64, 0.0f64).to_point(), radii: Vec2::splat(f64::INFINITY), start_angle: 0.0f64, sweep_angle: 1.0f64, x_rotation: 0.0f64 }, 2.220446049250313e-16f64);
}
