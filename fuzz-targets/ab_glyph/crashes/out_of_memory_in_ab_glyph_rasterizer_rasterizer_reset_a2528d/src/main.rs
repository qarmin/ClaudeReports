use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let mut rasterizer = Rasterizer::new(0usize, 0usize);
    rasterizer.reset(1usize, 1000000000usize);
}
