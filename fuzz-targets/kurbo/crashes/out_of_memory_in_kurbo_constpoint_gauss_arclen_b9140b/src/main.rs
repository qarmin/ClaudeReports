use kurbo::BezPath;

fn main() {
    let _ = <kurbo::ConstPoint as kurbo::ParamCurveDeriv>::gauss_arclen(&<kurbo::Line as kurbo::ParamCurveDeriv>::deriv(&<kurbo::BezPath as kurbo::Shape>::as_line(&BezPath::with_capacity(1000000000usize)).unwrap()), &[]);
}
