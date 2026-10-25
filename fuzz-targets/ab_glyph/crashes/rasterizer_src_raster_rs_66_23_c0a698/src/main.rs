use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let mut rasterizer = Rasterizer::new(0usize, 0usize);
    rasterizer.reset(10000000000000000000usize, 255usize);
}
