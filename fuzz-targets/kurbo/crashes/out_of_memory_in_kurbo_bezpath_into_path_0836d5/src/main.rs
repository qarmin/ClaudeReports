use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::into_path(<kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment { center: Point::new(0.0f64, 0.0f64), outer_radius: 0.0f64, inner_radius: 0.0f64, start_angle: 0.0f64, sweep_angle: f64::NEG_INFINITY }, 0.0f64), 0.0f64);
}
