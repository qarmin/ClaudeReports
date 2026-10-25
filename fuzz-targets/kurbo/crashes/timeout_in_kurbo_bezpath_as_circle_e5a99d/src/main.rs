use kurbo::Circle;
use kurbo::MinDistance;
use kurbo::PathEl;
use kurbo::Point;
use kurbo::Size;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let circle = <kurbo::BezPath as kurbo::Shape>::as_circle(&<kurbo::CircleSegment as kurbo::Shape>::into_path(Circle::new(Point::from((0.841f32, f32::MIN)), 0.123f64).segment(10000000000.0f64, 1.7915043128019068e151f64, 0.236f64), 2.2250738585072014e-308f64)).unwrap();
    let _ = MinDistance { distance: 0.68f64, t1: 1e300f64, t2: -1.6184141932547296e-256f64 };
    let _ = Size::INFINITY;
    let _ = <kurbo::Circle as kurbo::Shape>::as_line(&circle);
    let _ = <kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit>::sample_pt_deriv(&SimplifyBezPath::new(vec![PathEl::ClosePath]), 2.2250738585072014e-308f64);
}
