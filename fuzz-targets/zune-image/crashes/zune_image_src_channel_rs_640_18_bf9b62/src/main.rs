use zune_core::bit_depth::BitDepth;
use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let color_space = ColorSpace::MultiBand(std::num::NonZero::new(1u32).unwrap());
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::clone_and_execute(&ColorspaceConv::new(color_space), &Image::new(vec![Channel::new_with_bit_type(2147483648usize, BitType::U8), Channel::new_with_bit_type(0usize, BitType::F32)], BitDepth::Float32, 0usize, 0usize, color_space));
}
