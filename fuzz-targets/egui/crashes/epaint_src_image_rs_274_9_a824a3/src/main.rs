use epaint::ColorImage;

fn main() {
    let _ = ColorImage::default().region_by_pixels([0usize, 0usize], [1usize, 0usize]);
}
