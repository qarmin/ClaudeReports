use zune_core::bit_depth::BitDepth;
use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::channel::MIN_ALIGNMENT;
use zune_image::codecs::qoi::QoiEncoder;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;

fn main() {
    let usize_v = MIN_ALIGNMENT;
    let mut image = Image::new(vec![Channel::new_with_bit_type(usize_v, BitType::U16)], BitDepth::Float32, 0usize, 0usize, ColorSpace::Luma);
    let _ = <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute_impl(&Depth::new(<zune_image::codecs::qoi::QoiEncoder as zune_image::traits::EncoderTrait>::default_depth(&QoiEncoder::default(), BitDepth::Float32)), &mut image);
}
