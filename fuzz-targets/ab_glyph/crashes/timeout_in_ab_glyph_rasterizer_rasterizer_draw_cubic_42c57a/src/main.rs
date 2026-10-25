use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let mut rasterizer = Rasterizer::new(0usize, usize::MAX);
    rasterizer.draw_cubic(<ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(Point { x: 2.8e16f32, y: 1e-40f32 }, Point { x: f32::NAN, y: -2187873800.0f32 }), Point::default(), Point { x: -1e30f32, y: 1e30f32 }, <ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(ab_glyph_rasterizer::point(1.0f32, -6.4f32), ab_glyph_rasterizer::point(65536.0f32, 5.2e-6f32)));
}
