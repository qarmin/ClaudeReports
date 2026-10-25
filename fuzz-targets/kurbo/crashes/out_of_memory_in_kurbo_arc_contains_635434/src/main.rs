use kurbo::BezPath;
use kurbo::Point;

fn main() {
    let _ = <kurbo::Arc as kurbo::Shape>::contains(&<kurbo::BezPath as kurbo::Shape>::as_circle(&BezPath::with_capacity(1000000000usize)).unwrap().segment(0.0f64, 0.0f64, 0.0f64).inner_arc(), Point::new(0.0f64, 0.0f64));
}
