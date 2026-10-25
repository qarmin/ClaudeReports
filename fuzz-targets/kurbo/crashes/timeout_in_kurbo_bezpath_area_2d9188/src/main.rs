use kurbo::Arc;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let point = Point::from((0.0f32, 55433.484f32));
    let _ = <kurbo::BezPath as kurbo::Shape>::area(&kurbo::expand_path(Arc { center: point, radii: Vec2::splat(f64::NAN), start_angle: f64::NEG_INFINITY, sweep_angle: 9007199254740992.0f64, x_rotation: 9007199254740992.0f64 }, <kurbo::Diagonal2 as ::core::ops::Neg>::neg(Diagonal2::new(255.0f64, 65536.0f64)), Join::Round, 0.796f64, f64::NAN));
}
