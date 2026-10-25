use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let mut rasterizer = Rasterizer::new(1usize, 10000usize);
    rasterizer.draw_line(ab_glyph_rasterizer::point(f32::MAX, 356.0f32), Point::default());
}
