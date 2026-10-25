use kurbo::CircleSegment;
use kurbo::Ellipse;
use kurbo::PathEl;
use kurbo::Point;
use kurbo::QuadBez;
use kurbo::TranslateScale;
use kurbo::Vec2;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let path_el = PathEl::CurveTo(Point::from((0.0f32, 957.0f32)), <kurbo::QuadBez as kurbo::ParamCurve>::start(&QuadBez { p0: Point::ORIGIN, p1: Point::ZERO, p2: Ellipse::default().center() }), <kurbo::TranslateScale as ::core::ops::Mul<kurbo::Point>>::mul(TranslateScale::new(Vec2 { x: 0.0f64, y: 0.0f64 }, 255.0f64), Point::from((256.0f32, 1.1920929e-7f32)).trunc()));
    let simplify_bez_path = SimplifyBezPath::new(vec![path_el, path_el, path_el, path_el]);
    let _ = CircleSegment::new(kurbo::fit_to_bezpath_opt(&simplify_bez_path, 2.220446049250313e-16f64).current_position().unwrap(), 1e-310f64, 0.5f64, 9007199254740992.0f64, -0.0f64);
}
