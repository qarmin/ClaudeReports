use kurbo::Triangle;

fn main() {
    let triangle = Triangle::from_coords((0.0f64, 0.0f64), (0.0f64, 1.0f64), (0.0f64, 1.0f64));
    let circle = triangle.circumscribed_circle();
    let _ = <kurbo::CircleSegment as kurbo::Shape>::to_path(&circle.segment(1.0f64, 0.0f64, 1.0f64), 0.0f64);
}
