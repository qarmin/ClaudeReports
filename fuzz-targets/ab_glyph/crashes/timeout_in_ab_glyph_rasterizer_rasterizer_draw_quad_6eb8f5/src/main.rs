use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let mut rasterizer = Rasterizer::new(0usize, 10000usize);
    rasterizer.draw_quad(ab_glyph_rasterizer::point(f32::INFINITY, 65536.0f32), ab_glyph_rasterizer::point(0.0f32, f32::NAN), Point::default());
}
