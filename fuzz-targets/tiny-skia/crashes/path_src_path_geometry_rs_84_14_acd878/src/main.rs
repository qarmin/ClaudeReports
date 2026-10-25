use tiny_skia::Color;
use tiny_skia_path::NormalizedF32Exclusive;
use tiny_skia_path::Point;

fn main() {
    let color = Color::from_rgba8(0u8, 0u8, 0u8, 0u8);
    let premultiplied_color = color.premultiply();
    let f32_v = premultiplied_color.blue();
    tiny_skia_path::path_geometry::chop_quad_at(&[], NormalizedF32Exclusive::ANY, &mut [Point { x: f32_v, y: 0.0f32 }, Point::default(), Point::zero(), Point::from_xy(0.0f32, f32_v), Point { x: 0.0f32, y: 0.0f32 }]);
}
