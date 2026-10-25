use kurbo::Axis;
use kurbo::BezPath;
use kurbo::PathEl;
use kurbo::Point;

fn main() {
    let mut bez_path = BezPath::with_capacity(0usize);
    bez_path.push(PathEl::QuadTo(Axis::Horizontal.pack_point(0.0f64, 0.0f64), Point::ZERO));
}
