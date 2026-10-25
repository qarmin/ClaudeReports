use kurbo::Circle;
use kurbo::Point;

fn main() {
    let circle = Circle::new(Point::ZERO, 1.0f64);
    let _ = <kurbo::Line as kurbo::Shape>::to_path(&<kurbo::BezPath as kurbo::Shape>::as_line(&<kurbo::Circle as kurbo::Shape>::to_path(&circle, 0.0f64)).unwrap(), 0.0f64);
}
