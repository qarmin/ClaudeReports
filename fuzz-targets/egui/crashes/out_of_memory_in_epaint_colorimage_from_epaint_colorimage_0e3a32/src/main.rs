use ecolor::Color32;
use epaint::ColorImage;

fn main() {
    let _ = ColorImage::from(ColorImage::filled([1000000000usize, 1usize], Color32::from_rgba_unmultiplied(1u8, 0u8, 0u8, 0u8)));
}
