use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::farbfeld::FarbFeldEncoder;
use zune_image::image::Image;

fn main() {
    let mut farb_feld_encoder = FarbFeldEncoder::new();
    let _ = <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut farb_feld_encoder, &Image::new_frames(vec![], BitDepth::Unknown, 0usize, 0usize, ColorSpace::YCbCr));
}
