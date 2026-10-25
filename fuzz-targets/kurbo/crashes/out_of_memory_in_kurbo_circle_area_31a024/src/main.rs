use kurbo::BezPath;

fn main() {
    let _ = <kurbo::Circle as kurbo::Shape>::area(&<kurbo::Line as kurbo::Shape>::as_circle(&<kurbo::BezPath as kurbo::Shape>::as_line(&BezPath::with_capacity(1000000000usize)).unwrap()).unwrap());
}
