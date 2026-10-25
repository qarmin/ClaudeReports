use ecolor::Color32;
use emath::Vec2;
use epaint::ColorImage;

fn main() {
    let color_image = ColorImage { size: [usize::MAX, 18446744073709551614usize], source_size: Vec2::from((1.0000001f32, -1.0f32)), pixels: vec![Color32::BLACK, Color32::GREEN, Color32::from_additive_luminance(86u8)] };
    let _ = color_image.region_by_pixels([16777214usize, 65536usize], [31usize, 8191usize]);
}
