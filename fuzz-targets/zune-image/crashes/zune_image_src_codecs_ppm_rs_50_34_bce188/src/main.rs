use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::ppm::PPMEncoder;
use zune_image::image::Image;

fn main() {
    let mut p_p_m_encoder = PPMEncoder::new();
    let _ = <zune_image::codecs::ppm::PPMEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut p_p_m_encoder, &Image::new_frames(vec![], BitDepth::Float32, 0usize, 0usize, ColorSpace::RGBA));
}
