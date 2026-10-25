use kurbo::BezPath;
use kurbo::CircleSegment;

fn main() {
    let _ = CircleSegment { center: <kurbo::PathSeg as kurbo::ParamCurve>::eval(&BezPath::with_capacity(1000000000usize).get_seg(0usize).unwrap(), 0.0f64), outer_radius: 0.0f64, inner_radius: 0.0f64, start_angle: 0.0f64, sweep_angle: 0.0f64 };
}
