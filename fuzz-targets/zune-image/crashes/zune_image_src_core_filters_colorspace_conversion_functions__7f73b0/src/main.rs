use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::ppm::PPMEncoder;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;

fn main() {
    let mut image = Image::new_frames(vec![], <zune_image::codecs::ppm::PPMEncoder as zune_image::traits::EncoderTrait>::default_depth(&PPMEncoder::new(), BitDepth::Unknown), 0usize, 0usize, ColorSpace::HSL);
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute(&ColorspaceConv::new(ColorSpace::LumaA), &mut image);
}
