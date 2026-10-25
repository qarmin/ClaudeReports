use epaint::ColorImage;

fn main() {
    let _ = ColorImage::default().region_by_pixels([1usize, 0usize], [usize::MAX, 0usize]);
}
