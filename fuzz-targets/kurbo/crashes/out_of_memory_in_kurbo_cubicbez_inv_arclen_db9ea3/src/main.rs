use kurbo::BezPath;

fn main() {
    let _ = <kurbo::CubicBez as kurbo::ParamCurveArclen>::inv_arclen(&BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap().to_cubic(), 0.0f64, 0.0f64);
}
