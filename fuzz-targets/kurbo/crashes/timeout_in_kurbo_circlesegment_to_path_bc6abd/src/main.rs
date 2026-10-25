use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let _ = <kurbo::CircleSegment as kurbo::Shape>::to_path(&CircleSegment::new(Point::ORIGIN, 0.0f64, 1.0f64, 9007199254740992.0f64, 1.0f64), 0.0f64);
}
