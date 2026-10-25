use kurbo::Affine;
use kurbo::Arc;
use kurbo::Axis;
use kurbo::Ellipse;
use kurbo::Moments;
use kurbo::Point;
use kurbo::StrokeCtx;
use kurbo::Vec2;
use kurbo::simplify::SimplifyOptLevel;

fn main() {
    let bez_path = <kurbo::Ellipse as kurbo::Shape>::into_path(Ellipse::default().with_radii(Affine::new([486.0f64, -9.36676457517332e-32f64, 3.914127681039034e297f64, 2.2250738585072014e-308f64, 0.9999999999999999f64, 0.0f64]).then_rotate_about(0.29f64, Point::ZERO).translation()), 1e-310f64);
    let axis = Axis::Vertical;
    let axis2 = axis.cross();
    let _ = axis2.cross();
    let _ = SimplifyOptLevel::Optimize;
    let _ = <kurbo::PathSeg as kurbo::Shape>::as_circle(&bez_path.get_seg(255usize).unwrap());
    let _ = Arc::new(Vec2::from((68.9f64, 267.0f64)).to_point(), Vec2::from((-56.4f64, 704.0f64)), 29.7f64, f64::MAX, 10000000000.0f64);
    let _ = Moments::default();
    let _ = StrokeCtx::default();
}
