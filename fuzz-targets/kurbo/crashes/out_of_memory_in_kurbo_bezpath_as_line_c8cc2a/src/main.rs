use kurbo::Triangle;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::as_line(&<kurbo::CircleSegment as kurbo::Shape>::to_path(&Triangle::from_coords((0.0f64, 0.0f64), (0.0f64, 0.0f64), (0.0f64, 0.0f64)).inscribed_circle().segment(1.0f64, 1.0f64, 1.0f64), 0.0f64));
}
