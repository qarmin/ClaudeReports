use zune_core::colorspace::ColorSpace;
use zune_image::codecs::hdr::HdrEncoder;
use zune_image::image::Image;

fn main() {
    let mut hdr_encoder = HdrEncoder::default();
    let _ = <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut hdr_encoder, &Image::fill::<u16>(1u16, ColorSpace::ARGB, 0usize, 0usize));
}
