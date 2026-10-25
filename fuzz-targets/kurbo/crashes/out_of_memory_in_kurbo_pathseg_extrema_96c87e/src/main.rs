use kurbo::BezPath;

fn main() {
    let _ = <kurbo::PathSeg as kurbo::ParamCurveExtrema>::extrema(&BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap());
}
