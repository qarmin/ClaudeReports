use zune_core::colorspace::ColorSpace;
use zune_image::codecs::qoi::QoiEncoder;
use zune_image::codecs::webp::ZuneWebpImageEncoder;
use zune_image::image::Image;
use zune_image::metadata::ImageMetadata;

fn main() {
    let bit_depth = <zune_image::codecs::webp::ZuneWebpImageEncoder as zune_image::traits::EncoderTrait>::default_depth(&ZuneWebpImageEncoder::new(), ImageMetadata::default().depth());
    let mut qoi_encoder = QoiEncoder::new();
    let _ = <zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait>::encode_to_result(&mut qoi_encoder, &Image::new_frames(vec![], bit_depth, 1usize, 0usize, ColorSpace::HSV));
}
