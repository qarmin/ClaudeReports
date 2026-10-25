use kurbo::Axis;
use kurbo::Cap;
use kurbo::CircleSegment;
use kurbo::Ellipse;
use kurbo::Join;
use kurbo::MinDistance;
use kurbo::Point;
use kurbo::QuadBez;
use kurbo::Stroke;
use kurbo::StrokeOptLevel;
use kurbo::TranslateScale;
use kurbo::simplify::SimplifyOptions;

fn main() {
    let _ = StrokeOptLevel::Subdivide;
    let stroke = Stroke::default();
    let stroke2 = stroke.clone().with_caps(Cap::Butt);
    let stroke3 = stroke2.clone().with_join(Join::Miter);
    let _ = stroke3.clone().with_caps(Cap::Square);
    let _ = MinDistance { distance: 7.506104286755182e39f64, t1: -2.1348825792426762e278f64, t2: 1.199469914557974e245f64 };
    let axis = Axis::Horizontal;
    let axis2 = axis.cross();
    let axis3 = axis2.cross();
    let point = axis3.pack_point(f64::MIN, -51.8f64);
    let _ = <kurbo::QuadBez as kurbo::Shape>::as_rounded_rect(&<kurbo::TranslateScale as ::core::ops::Mul<kurbo::QuadBez>>::mul(TranslateScale::scale(f64::NAN), QuadBez::new::<kurbo::Point>(point, point, Point::default())));
    let _ = SimplifyOptions::default();
    let _ = Ellipse::from_rect(<kurbo::Arc as kurbo::Shape>::bounding_box(&CircleSegment::new(point, 237.0f64, 57.4f64, -1e300f64, 1e300f64).outer_arc()));
}
