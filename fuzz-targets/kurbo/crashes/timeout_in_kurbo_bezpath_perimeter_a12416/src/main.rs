use kurbo::CircleSegment;
use kurbo::Triangle;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::perimeter(&<kurbo::CircleSegment as kurbo::Shape>::to_path(&CircleSegment::new(Triangle::EQUILATERAL.centroid(), 0.0f64, 0.0f64, 25800000000.0f64, 10000000000.0f64), 75.50000000000001f64), 65536.0f64);
}
