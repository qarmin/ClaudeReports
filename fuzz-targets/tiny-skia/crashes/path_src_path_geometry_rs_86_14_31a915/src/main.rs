use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::PathBuilder;
use tiny_skia_path::Point;

fn main() {
    let f32x2 = tiny_skia_path::f32x2::splat(0.0f32);
    let mut path_builder = PathBuilder::new();
    path_builder.line_to(0.0f32, 0.0f32);
    tiny_skia_path::path_geometry::chop_quad_at(&[Point::from_f32x2(f32x2), Point::from_xy(0.0f32, 0.0f32)], NormalizedF32Exclusive::ANY, &mut [Point { x: 0.0f32, y: 0.0f32 }, path_builder.last_point().unwrap(), Point::default(), path_builder.last_point().unwrap(), path_builder.last_point().unwrap()]);
}
