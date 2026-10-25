use kurbo::CircleSegment;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;

fn main() {
    let _ = kurbo::expand_path_signed(CircleSegment::new(Point::from((0.0f32, 0.0f32)), 1.0f64, 1.0f64, 0.0f64, 1.0f64).outer_arc(), Diagonal2 { xx: 0.0f64, yy: 0.0f64 }, Join::Bevel, 0.0f64, 0.0f64);
}
