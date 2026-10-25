use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::Point;
use tiny_skia_path::SCALAR_MAX;

fn main() {
    let f32_v = SCALAR_MAX;
    let point = Point { x: f32_v, y: 0.0f32 };
    tiny_skia_path::path_geometry::chop_cubic_at2(&[point, point, point, point], NormalizedF32Exclusive::ANY, &mut vec![point, Point::from((0.0f32, 1.0f32)), point, point, Point::zero(), point]);
}
