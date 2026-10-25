use kurbo::BezPath;
use kurbo::Point;
use kurbo::QuadBez;

fn main() {
    let _ = <kurbo::QuadBez as kurbo::ParamCurveExtrema>::extrema(&QuadBez { p0: Point::from((0.0f32, 0.0f32)), p1: Point::ORIGIN, p2: BezPath::with_capacity(1000000000usize).current_position().unwrap() });
}
