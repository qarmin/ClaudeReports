use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::Point;

fn main() {
    let point = Point { x: 0.0f32, y: 0.0f32 };
    let f32_v = point.length();
    tiny_skia_path::path_geometry::chop_cubic_at2(&[Point { x: f32_v, y: 0.0f32 }, point, Point::from_xy(0.0f32, 1.0f32), point], NormalizedF32Exclusive::ANY, &mut vec![Point::from((0.0f32, 0.0f32)), point, point]);
}
