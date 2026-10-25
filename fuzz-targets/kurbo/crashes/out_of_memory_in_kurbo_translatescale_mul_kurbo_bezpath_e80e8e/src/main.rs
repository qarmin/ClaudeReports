use kurbo::CircleSegment;
use kurbo::Point;
use kurbo::TranslateScale;

fn main() {
    let _ = <kurbo::TranslateScale as ::core::ops::Mul<kurbo::BezPath>>::mul(TranslateScale::from_scale_about(0.0f64, Point::from((0.0f32, 0.0f32))), <kurbo::CircleSegment as kurbo::Shape>::into_path(CircleSegment::new(Point::ORIGIN, 0.0f64, 1.0f64, 0.0f64, 1.0f64), 0.0f64));
}
