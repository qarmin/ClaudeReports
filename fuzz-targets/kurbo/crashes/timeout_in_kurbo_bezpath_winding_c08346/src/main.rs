use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::winding(&<kurbo::CircleSegment as kurbo::Shape>::to_path(&CircleSegment { center: <kurbo::Point as ::core::ops::Sub<(f64, f64)>>::sub(Point::from((256.0f32, 384.0f32)), (2.2250738585072014e-308f64, 1.0f64)), outer_radius: 255.0f64, inner_radius: 166.0f64, start_angle: -1e300f64, sweep_angle: f64::MAX }, 585.0f64), Point::new(f64::NAN, 0.5f64).ceil());
}
