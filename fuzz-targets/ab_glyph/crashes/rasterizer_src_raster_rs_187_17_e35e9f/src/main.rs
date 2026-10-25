use ab_glyph::Rect;
use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let rect = Rect { min: ab_glyph_rasterizer::point(1.0f32, 0.0f32), max: Point::default() };
    let f32_v = rect.width();
    let mut rasterizer = Rasterizer::new(0usize, 1usize);
    rasterizer.draw_quad(ab_glyph_rasterizer::point(0.0f32, f32_v), Point::default(), Point { x: 1e30f32, y: 0.0f32 });
}
