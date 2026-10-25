use kurbo::BezPath;

fn main() {
    let _ = <kurbo::ConstPoint as kurbo::ParamCurve>::subdivide(&<kurbo::Line as kurbo::ParamCurveDeriv>::deriv(&<kurbo::Rect as kurbo::Shape>::as_line(&<kurbo::BezPath as kurbo::Shape>::as_rect(&BezPath::with_capacity(1000000000usize)).unwrap()).unwrap()));
}
