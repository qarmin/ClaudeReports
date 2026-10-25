use kurbo::Axis;
use kurbo::CircleSegment;

fn main() {
    let axis = Axis::Vertical;
    let point = axis.pack_point(0.0f64, 0.0f64);
    let _ = <kurbo::Arc as kurbo::Shape>::into_path(CircleSegment { center: point, outer_radius: 0.0f64, inner_radius: 0.0f64, start_angle: 0.0f64, sweep_angle: f64::MAX }.inner_arc(), 0.0f64);
}
