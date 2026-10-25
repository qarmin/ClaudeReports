use kurbo::BezPath;

fn main() {
    let _ = <kurbo::Circle as kurbo::Shape>::as_path_slice(&<kurbo::BezPath as kurbo::Shape>::as_circle(&BezPath::with_capacity(1000000000usize)).unwrap());
}
