use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;
use zune_image::metadata::ImageMetadata;

fn main() {
    let mut image = Image::new(vec![Channel::new_with_bit_type(0usize, BitType::F32), Channel::new_with_bit_type(0usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::F32)], ImageMetadata::default().depth(), 0usize, 0usize, ColorSpace::CMYK);
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl(&ColorspaceConv::new(ColorSpace::Unknown), &mut image);
}
