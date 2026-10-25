use kurbo::Axis;
use kurbo::CircleSegment;

fn main() {
    let axis = Axis::Vertical;
    let point = axis.pack_point(0.0f64, 0.0f64);
    let _ = <kurbo::PathSeg as kurbo::ParamCurveNearest>::nearest(&<kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment::new(point, 0.0f64, 0.0f64, 0.0f64, f64::INFINITY), 0.0f64).get_seg(0usize).unwrap(), point, 0.0f64);
}
