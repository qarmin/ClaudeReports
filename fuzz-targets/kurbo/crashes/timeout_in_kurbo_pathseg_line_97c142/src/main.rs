use kurbo::Affine;
use kurbo::Arc;
use kurbo::Axis;
use kurbo::Line;
use kurbo::PathSeg;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let axis = Axis::Horizontal;
    let axis2 = axis.cross();
    let _ = axis2.cross();
    let arc = Arc::new(Point::new(f64::MAX, -0.0f64), Vec2::new(-1.0f64, f64::NAN), f64::MAX, 2.5598864519886808e92f64, -1e300f64);
    let arc2 = arc.reversed();
    let arc3 = <kurbo::Affine as ::core::ops::Mul<kurbo::Arc>>::mul(Affine::scale(-1.5391913684270659e263f64), arc2);
    let _ = PathSeg::Line(Line::from((<kurbo::Arc as kurbo::Shape>::bounding_box(&arc3).origin(), Vec2::splat(66.8f64))));
}
