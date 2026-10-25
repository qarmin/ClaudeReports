use kurbo::BezPath;
use kurbo::CircleSegment;

fn main() {
    let _ = CircleSegment::new(BezPath::with_capacity(100000000000usize).current_position().unwrap(), 0.0f64, 0.0f64, 0.0f64, 0.0f64);
}
