use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;
use zune_image::image::MAX_CHANNELS;

fn main() {
    let usize_v = MAX_CHANNELS;
    let _ = Image::fill::<u16>(0u16, ColorSpace::MultiBand(std::num::NonZero::new(10000u32).unwrap()), usize_v, 65536usize);
}
