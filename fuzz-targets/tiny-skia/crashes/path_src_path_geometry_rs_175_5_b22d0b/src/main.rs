use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::Point;

fn main() {
    let point = Point::from_xy(0.0f32, 0.0f32);
    tiny_skia_path::path_geometry::chop_cubic_at2(&[point, point, Point { x: 0.0f32, y: 0.0f32 }, point], NormalizedF32Exclusive::ANY, &mut vec![point, point]);
}
