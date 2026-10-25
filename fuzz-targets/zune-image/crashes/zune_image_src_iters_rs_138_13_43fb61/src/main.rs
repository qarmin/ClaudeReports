use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let image = Image::fill::<u16>(0u16, ColorSpace::MultiBand(std::num::NonZero::new(1000u32).unwrap()), 0usize, 0usize);
    let _ = image.pixels::<u16>();
}
