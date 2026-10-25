use kurbo::CircleSegment;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;

fn main() {
    let _ = kurbo::expand_path_signed(<kurbo::CircleSegment as kurbo::Shape>::to_path(&CircleSegment::new(Point::ORIGIN, 0.0f64, 0.0f64, f64::MAX, f64::MAX), 0.0f64).get_seg(0usize).unwrap(), Diagonal2 { xx: 0.0f64, yy: 0.0f64 }, Join::Round, f64::NAN, f64::NEG_INFINITY);
}
