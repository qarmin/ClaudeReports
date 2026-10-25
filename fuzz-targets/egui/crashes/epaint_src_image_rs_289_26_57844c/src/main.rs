use emath::Vec2;
use epaint::ColorImage;
use epaint::FontColorTransferFunction;
use epaint::text::MAX_GLYPH_SIZE;

fn main() {
    let usize_v = MAX_GLYPH_SIZE;
    let color_image = ColorImage { size: [usize::MAX, usize::MAX], source_size: Vec2::UP, pixels: vec![FontColorTransferFunction::Gamma(-1.1754944e-38f32).color_from_coverage(f32::MIN)] };
    let _ = color_image.region_by_pixels([usize_v, 0usize], [1usize, 18446744073709551614usize]);
}
