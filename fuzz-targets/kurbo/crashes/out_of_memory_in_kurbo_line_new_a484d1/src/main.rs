use kurbo::BezPath;
use kurbo::Line;
use kurbo::Point;

fn main() {
    let _ = Line::new(Point::from((0.0f32, 0.0f32)), BezPath::with_capacity(100000000usize).current_position().unwrap());
}
