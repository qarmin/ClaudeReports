use kurbo::Arc;
use kurbo::Diagonal2;
use kurbo::PathEl;
use kurbo::Point;
use kurbo::Vec2;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let diagonal2 = Diagonal2::new(0.0f64, 0.0f64);
    let vec2 = diagonal2.scale_normal(Vec2::new(0.0f64, 0.0f64));
    let point = <kurbo::Arc as kurbo::ParamCurve>::end(&Arc { center: Point::ORIGIN, radii: vec2, start_angle: 0.0f64, sweep_angle: 0.0f64, x_rotation: 0.0f64 });
    let path_el = PathEl::QuadTo(point.expand(), point);
    let _ = kurbo::simplify::simplify_bezpath(vec![path_el], 0.0f64, &SimplifyOptions::default());
}
