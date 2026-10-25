use kurbo::BezPath;
use kurbo::Circle;

fn main() {
    let _ = <kurbo::Circle as kurbo::Shape>::as_circle(&Circle::new(BezPath::with_capacity(1000000000usize).current_position().unwrap(), 0.0f64));
}
