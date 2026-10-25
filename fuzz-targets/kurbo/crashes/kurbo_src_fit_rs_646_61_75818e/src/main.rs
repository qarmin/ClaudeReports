use kurbo::Line;
use kurbo::PathEl;
use kurbo::Point;
use kurbo::Vec2;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let point = Point::from((0.0f64, 0.0f64));
    let f64_v = <kurbo::Line as kurbo::ParamCurveArclen>::arclen(&Line::from((Point { x: 0.0f64, y: 0.0f64 }, Vec2 { x: 0.0f64, y: 0.0f64 })), 0.0f64);
    let _ = kurbo::fit_to_bezpath_opt(&SimplifyBezPath::new(vec![PathEl::QuadTo(point, Point::new(f64_v, 1.0f64))]), 0.0f64);
}
