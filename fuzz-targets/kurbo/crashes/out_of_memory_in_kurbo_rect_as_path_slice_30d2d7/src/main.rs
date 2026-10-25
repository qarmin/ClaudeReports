use kurbo::BezPath;

fn main() {
    let _ = <kurbo::Rect as kurbo::Shape>::as_path_slice(&<kurbo::BezPath as kurbo::Shape>::bounding_box(&BezPath::with_capacity(1000000000usize)));
}
