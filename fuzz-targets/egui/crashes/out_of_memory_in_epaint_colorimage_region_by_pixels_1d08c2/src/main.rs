use emath::Vec2;
use epaint::ColorImage;

fn main() {
    let color_image = ColorImage { size: [10000000000000usize, 100000000000000000usize], source_size: Vec2::RIGHT, pixels: vec![] };
    let color_image2 = color_image.clone().with_source_size(Vec2::RIGHT);
    let _ = color_image2.region_by_pixels([1usize, 0usize], [1000000usize, 10000usize]);
}
