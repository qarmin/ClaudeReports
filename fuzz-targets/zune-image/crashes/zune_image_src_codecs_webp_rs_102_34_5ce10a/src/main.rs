use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::hdr::HdrEncoder;
use zune_image::codecs::jpeg::JpegEncoder;
use zune_image::codecs::webp::ZuneWebpImageEncoder;
use zune_image::image::Image;

fn main() {
    let mut zune_webp_image_encoder = ZuneWebpImageEncoder::default();
    let _ = <zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut zune_webp_image_encoder, &Image::new_frames(vec![], <zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::EncoderTrait>::default_depth(&JpegEncoder::new(), BitDepth::Unknown), 0usize, 0usize, <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::default_colorspace(&HdrEncoder::default(), ColorSpace::YCbCr)));
}
