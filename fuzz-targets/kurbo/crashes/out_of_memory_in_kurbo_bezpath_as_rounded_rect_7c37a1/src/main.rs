use kurbo::BezPath;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::as_rounded_rect(&BezPath::with_capacity(1000000000usize));
}
