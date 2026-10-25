use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let point = Point::default();
    let mut rasterizer = Rasterizer::new(1000usize, 2usize);
    rasterizer.draw_cubic(ab_glyph_rasterizer::point(0.0f32, 0.0f32), point, <ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(ab_glyph_rasterizer::point(0.0f32, 0.0f32), Point { x: 256.0f32, y: 0.0f32 }), ab_glyph_rasterizer::point(0.0f32, 13.0f32));
}
