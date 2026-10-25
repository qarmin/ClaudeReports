use kurbo::BezPath;

fn main() {
    let _ = <kurbo::Line as kurbo::ParamCurve>::subdivide(&<kurbo::BezPath as kurbo::Shape>::as_line(&BezPath::with_capacity(1000000000usize)).unwrap());
}
