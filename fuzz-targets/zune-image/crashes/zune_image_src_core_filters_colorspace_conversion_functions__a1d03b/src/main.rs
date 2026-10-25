use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let mut image = Image::new(vec![], BitDepth::Float32, 0usize, 1usize, <zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>> as zune_image::traits::DecoderTrait>::out_colorspace(&<zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(vec![82u8, 73u8, 70u8, 70u8, 26u8, 0u8, 0u8, 0u8, 87u8, 69u8, 66u8, 80u8, 86u8, 80u8, 56u8, 76u8, 13u8, 0u8, 0u8, 0u8, 47u8, 0u8, 0u8, 0u8, 16u8])).unwrap()));
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl(&ColorspaceConv::new(ColorSpace::RGB), &mut image);
}
