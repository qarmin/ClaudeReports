use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::jpeg_xl::JxlEncoder;
use zune_image::image::Image;

fn main() {
    let mut jxl_encoder = JxlEncoder::new();
    let _ = <zune_image::codecs::jpeg_xl::JxlEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut jxl_encoder, &Image::new_frames(vec![], BitDepth::Float32, 0usize, 0usize, ColorSpace::RGB));
}
