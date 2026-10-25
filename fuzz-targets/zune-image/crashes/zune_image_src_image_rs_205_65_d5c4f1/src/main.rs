use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::farbfeld::FarbFeldEncoder;
use zune_image::image::Image;

fn main() {
    let mut image = Image::fill::<f32>(0.0f32, ColorSpace::RGBA, 0usize, 0usize);
    image.set_depth(BitDepth::Sixteen);
    let mut farb_feld_encoder = FarbFeldEncoder::new();
    let _ = <zune_image::codecs::farbfeld::FarbFeldEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut farb_feld_encoder, &image);
}
