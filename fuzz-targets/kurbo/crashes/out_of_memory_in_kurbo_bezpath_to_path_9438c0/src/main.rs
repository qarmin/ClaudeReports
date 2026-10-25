use kurbo::Ellipse;
use kurbo::Point;

fn main() {
    let vec2 = <kurbo::Vec2 as ::core::ops::Div<f64>>::div(Point::new(1.0f64, 0.0f64).to_vec2(), 1.0f64);
    let vec22 = vec2.round();
    let point = Point::new(0.0f64, 0.0f64);
    let f64_v = point.distance(Point::from((0.0f64, 0.0f64)));
    let _ = <kurbo::BezPath as kurbo::Shape>::to_path(&<kurbo::Ellipse as kurbo::Shape>::to_path(&Ellipse::new(point, vec22, f64_v), -0.0f64), f64_v);
}
