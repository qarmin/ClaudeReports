use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::qoi::QoiEncoder;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let mut image = Image::fill::<f32>(0.0f32, ColorSpace::HSL, 0usize, 0usize);
    image.set_depth(<zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait>::default_depth(&QoiEncoder::default(), BitDepth::Float32));
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute(&ColorspaceConv::new(ColorSpace::CMYK), &mut image);
}
