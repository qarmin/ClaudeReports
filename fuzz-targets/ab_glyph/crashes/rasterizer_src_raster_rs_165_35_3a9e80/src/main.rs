use ab_glyph_rasterizer::Point;
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let mut rasterizer = Rasterizer::new(1usize, 10000usize);
    rasterizer.draw_cubic(Point { x: 1.0f32, y: 65536.0f32 }, Point { x: 0.0f32, y: -7110000000.0f32 }, Point { x: 0.0f32, y: 0.0f32 }, Point { x: -3.736144e16f32, y: 0.0f32 });
}
