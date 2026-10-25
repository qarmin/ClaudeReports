use kurbo::Arc;
use kurbo::Axis;
use kurbo::LineIntersection;
use kurbo::RoundedRect;
use kurbo::SvgArc;
use kurbo::Vec2;

fn main() {
    let axis = Axis::Vertical;
    let point = axis.pack_point(0.0f64, 2.2250738585072014e-308f64);
    let line_intersection = LineIntersection { line_t: 0.0f64, segment_t: 0.0f64 };
    let bool_v = line_intersection.is_nan();
    let _ = Arc::from_svg_arc(&SvgArc { from: RoundedRect::default().origin(), to: point, radii: Vec2::from_angle(1.0f64), x_rotation: 0.0f64, large_arc: bool_v, sweep: false });
}
