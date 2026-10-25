use kurbo::BezPath;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::as_circle(&BezPath::with_capacity(1000000000usize));
}
