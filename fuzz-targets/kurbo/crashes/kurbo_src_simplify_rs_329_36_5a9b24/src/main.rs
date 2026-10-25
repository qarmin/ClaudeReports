use kurbo::CubicBez;
use kurbo::PathEl;
use kurbo::PathSeg;
use kurbo::Point;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let cubic_bez = CubicBez { p0: Point::from((0.0f32, 0.0f32)), p1: Point::default(), p2: PathEl::MoveTo(Point::default()).end_point().unwrap(), p3: Point::default() };
    let _ = kurbo::simplify::simplify_bezpath(vec![PathSeg::from(cubic_bez).as_path_el()], 0.0f64, &SimplifyOptions::default());
}
