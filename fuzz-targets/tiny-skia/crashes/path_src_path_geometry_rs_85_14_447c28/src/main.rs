use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::Point;

fn main() {
    tiny_skia_path::path_geometry::chop_quad_at(&[Point::from_xy(0.0f32, 0.0f32)], NormalizedF32Exclusive::ANY, &mut [Point::default(), Point::from_f32x2(tiny_skia_path::f32x2::new(1.0f32, 0.0f32)), Point { x: 0.0f32, y: 0.0f32 }, Point { x: 0.0f32, y: 0.0f32 }, Point { x: 0.0f32, y: 0.0f32 }]);
}
