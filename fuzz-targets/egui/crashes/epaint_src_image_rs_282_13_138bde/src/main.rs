use epaint::ColorImage;

fn main() {
    let color_image = ColorImage::example();
    let _ = color_image.region_by_pixels([0usize, 1usize], [0usize, usize::MAX]);
}
