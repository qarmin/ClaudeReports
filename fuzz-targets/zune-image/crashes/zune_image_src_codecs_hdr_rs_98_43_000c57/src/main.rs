use zune_core::colorspace::ColorSpace;
use zune_image::codecs::hdr::HdrEncoder;
use zune_image::codecs::webp::ZuneWebpImageEncoder;
use zune_image::image::Image;
use zune_image::metadata::ImageMetadata;

fn main() {
    let mut hdr_encoder = HdrEncoder::new();
    let _ = <zune_image::codecs::hdr::HdrEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut hdr_encoder, &Image::new_frames(vec![], <zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::EncoderTrait>::default_depth(&ZuneWebpImageEncoder::default(), ImageMetadata::default().depth()), 0usize, 0usize, ColorSpace::ARGB));
}
