use ecolor::Color32;
use epaint::ColorImage;

fn main() {
    let _ = ColorImage::filled([1usize, 0usize], Color32::KHAKI).region_by_pixels([1usize, 0usize], [0usize, 1usize]);
}
