use zune_core::bit_depth::BitDepth;
use zune_core::colorspace::ColorSpace;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;
use zune_image::metadata::ImageMetadata;

fn main() {
    let mut image = Image::new(vec![], ImageMetadata::default().depth(), 0usize, 1usize, ColorSpace::ARGB);
    let _ = <zune_image::core_filters::depth::Depth as zune_image::traits::OperationsTrait>::execute(&Depth::new(BitDepth::Float32), &mut image);
}
