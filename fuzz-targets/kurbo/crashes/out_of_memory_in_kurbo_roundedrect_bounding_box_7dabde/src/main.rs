use kurbo::BezPath;

fn main() {
    let _ = <kurbo::RoundedRect as kurbo::Shape>::bounding_box(&<kurbo::BezPath as kurbo::Shape>::as_rounded_rect(&BezPath::with_capacity(10000000000000000usize)).unwrap());
}
