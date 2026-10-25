use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let mut rasterizer = Rasterizer::new(0usize, 1usize);
    rasterizer.reset(2147483648usize, 2147483648usize);
}
