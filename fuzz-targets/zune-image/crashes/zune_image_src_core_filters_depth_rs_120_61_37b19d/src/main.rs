use zune_core::bit_depth::BitDepth;
use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;

fn main() {
    let mut image = Image::new(vec![Channel::new_with_bit_type(0usize, BitType::U16), Channel::new_with_bit_type(0usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::F32)], BitDepth::Eight, 0usize, 0usize, <zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>> as zune_image::traits::DecoderTrait>::out_colorspace(&<zune_image::codecs::webp::ZuneWebpDecoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(vec![82u8, 73u8, 70u8, 70u8, 0u8, 0u8, 0u8, 0u8, 87u8, 69u8, 66u8, 80u8, 86u8, 80u8, 56u8, 76u8, 0u8, 0u8, 0u8, 0u8, 47u8, 0u8, 0u8, 0u8, 0u8])).unwrap()));
    let _ = <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute_impl(&Depth::new(BitDepth::Sixteen), &mut image);
}
