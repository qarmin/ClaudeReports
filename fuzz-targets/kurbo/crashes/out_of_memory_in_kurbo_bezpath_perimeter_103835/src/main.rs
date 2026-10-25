use kurbo::BezPath;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::perimeter(&<kurbo::Circle as kurbo::Shape>::to_path(&<kurbo::BezPath as kurbo::Shape>::as_circle(&BezPath::with_capacity(1000000000usize)).unwrap(), 0.0f64), 0.0f64);
}
