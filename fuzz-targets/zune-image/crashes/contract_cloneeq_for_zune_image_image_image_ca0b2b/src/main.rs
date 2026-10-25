use zune_core::bit_depth::BitDepth;
use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::image::Image;

fn main() {
    let image = Image::new(vec![Channel::new_with_bit_type(10000usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::F32)], BitDepth::Eight, 0usize, 0usize, ColorSpace::ARGB);
    assert!(image.clone() == image, "clone is not equal to the original");
}
