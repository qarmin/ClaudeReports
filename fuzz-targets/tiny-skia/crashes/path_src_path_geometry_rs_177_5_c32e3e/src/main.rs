use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::Point;

fn main() {
    tiny_skia_path::path_geometry::chop_cubic_at2(&[Point::from((1.0f32, 0.0f32)), Point::from((0.0f32, 0.0f32)), Point::zero(), Point::from((0.0f32, 0.0f32))], NormalizedF32Exclusive::HALF, &mut vec![Point { x: 0.0f32, y: 0.0f32 }, Point::default(), Point::zero(), Point::default()]);
}
