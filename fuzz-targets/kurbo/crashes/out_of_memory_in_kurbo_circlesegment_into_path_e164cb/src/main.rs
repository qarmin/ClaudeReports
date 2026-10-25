use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let _ = <kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment::new(Point::ZERO, 0.0f64, 0.0f64, 0.0f64, f64::NEG_INFINITY), 0.0f64);
}
