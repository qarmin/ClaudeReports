use zune_core::colorspace::ColorSpace;
use zune_image::codecs::jpeg::JpegEncoder;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;

fn main() {
    let color_space = <zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::EncoderTrait>::default_colorspace(&JpegEncoder::default(), ColorSpace::BGRA);
    let _ = Depth::new(Image::from_f32(&[], usize::MAX, 2147483648usize, color_space).depth());
}
