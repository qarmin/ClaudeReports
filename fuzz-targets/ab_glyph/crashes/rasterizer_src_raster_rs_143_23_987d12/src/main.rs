use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let point = <ab_glyph_rasterizer::Point as ::core::ops::Sub<ab_glyph_rasterizer::Point>>::sub(Point { x: f32::MAX, y: 0.0f32 }, Point::default());
    let mut rasterizer = Rasterizer::new(0usize, 1usize);
    rasterizer.draw_line(Point { x: 0.0f32, y: 6189.0f32 }, point);
}
