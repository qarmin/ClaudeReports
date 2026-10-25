use kurbo::CircleSegment;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;

fn main() {
    let circle_segment = CircleSegment { center: Point::ZERO, outer_radius: 0.0f64, inner_radius: 1.0f64, start_angle: 0.0f64, sweep_angle: f64::INFINITY };
    let _ = kurbo::expand_path(circle_segment.outer_arc(), Diagonal2 { xx: 0.0f64, yy: 0.0f64 }, Join::Bevel, 0.0f64, 0.0f64);
}
