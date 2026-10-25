use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::jpeg::JpegEncoder;
use zune_image::image::Image;

fn main() {
    let mut jpeg_encoder = JpegEncoder::new();
    let _ = <zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut jpeg_encoder, &Image::new_frames(vec![], BitDepth::Sixteen, 0usize, 0usize, ColorSpace::ARGB));
}
