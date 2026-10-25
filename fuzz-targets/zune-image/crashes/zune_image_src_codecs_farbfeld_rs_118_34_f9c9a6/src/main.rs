use zune_core::bit_depth::BitDepth;
use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::codecs::farbfeld::FarbFeldEncoder;
use zune_image::image::Image;

fn main() {
    let color_space = ColorSpace::BGRA;
    let channel = Channel::new_with_bit_type(0usize, BitType::U8);
    let usize_v = channel.len();
    let bit_depth = BitDepth::Eight;
    let mut farb_feld_encoder = FarbFeldEncoder::new();
    let _ = <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut farb_feld_encoder, &Image::new_frames(vec![], bit_depth, 1usize, usize_v, color_space));
}
