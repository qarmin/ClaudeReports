use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::webp::ZuneWebpImageEncoder;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let mut image = Image::new(vec![], BitDepth::Sixteen, 0usize, 0usize, <zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::EncoderTrait>::default_colorspace(&ZuneWebpImageEncoder::default(), ColorSpace::RGBA));
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl(&ColorspaceConv::new(ColorSpace::CMYK), &mut image);
}
