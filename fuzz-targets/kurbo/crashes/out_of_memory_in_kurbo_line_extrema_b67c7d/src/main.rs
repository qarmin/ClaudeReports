use kurbo::BezPath;

fn main() {
    let _ = <kurbo::Line as kurbo::ParamCurveExtrema>::extrema(&<kurbo::BezPath as kurbo::Shape>::as_line(&BezPath::with_capacity(1000000000usize)).unwrap());
}
