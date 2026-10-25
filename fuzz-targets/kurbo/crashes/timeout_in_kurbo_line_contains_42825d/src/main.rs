use kurbo::Line;
use kurbo::Point;
use kurbo::Triangle;

fn main() {
    let triangle = Triangle::from_coords((-0.9000000000000004f64, 1e300f64), (1e-310f64, 43.1f64), (-6.68e-31f64, 9007199254740992.0f64));
    let circle = triangle.inscribed_circle();
    let circle_segment = circle.segment(f64::MAX, 7.450580596923553e-9f64, 810.0f64);
    let arc = circle_segment.inner_arc();
    let _ = <kurbo::Line as kurbo::Shape>::contains(&Line::new(Point::ORIGIN, (<kurbo::Arc as kurbo::Shape>::bounding_box(&<kurbo::Arc as kurbo::ParamCurve>::subsegment(&arc, 1e300f64..1e-310f64))).center()), <kurbo::CircleSegment as kurbo::Shape>::to_path(&circle_segment, -4.87e-19f64).current_position().unwrap());
}
