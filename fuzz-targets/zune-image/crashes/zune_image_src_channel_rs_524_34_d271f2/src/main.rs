use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let color_space = ColorSpace::RGBA;
    let _ = Image::fill::<u16>(0u16, color_space, 1usize, 4294967295usize);
}
