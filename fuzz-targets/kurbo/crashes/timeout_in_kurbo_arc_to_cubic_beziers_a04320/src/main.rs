use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let circle_segment = CircleSegment { center: Point::from((0.0f64, 0.0f64)), outer_radius: 1.0f64, inner_radius: f64::NAN, start_angle: 0.384f64, sweep_angle: 42.2f64 };
    let arc = circle_segment.outer_arc();
    arc.to_cubic_beziers::<fn(kurbo::Point, kurbo::Point, kurbo::Point)>(1e-310f64, |_, _, _| ());
}
