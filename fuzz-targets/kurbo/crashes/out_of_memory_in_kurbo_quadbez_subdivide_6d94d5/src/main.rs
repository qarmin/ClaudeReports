use kurbo::BezPath;
use kurbo::Point;
use kurbo::QuadBez;

fn main() {
    let _ = <kurbo::QuadBez as kurbo::ParamCurve>::subdivide(&QuadBez { p0: BezPath::with_capacity(1000000000usize).current_position().unwrap(), p1: Point { x: 0.0f64, y: 0.0f64 }, p2: Point::new(0.0f64, 0.0f64) });
}
