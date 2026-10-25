use ecolor::Color32;
use epaint::ColorImage;

fn main() {
    let _ = ColorImage::filled([1000000000usize, 1usize], Color32::from_rgb(0u8, 0u8, 0u8));
}
