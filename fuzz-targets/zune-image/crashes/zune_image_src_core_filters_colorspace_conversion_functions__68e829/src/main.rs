use zune_core::colorspace::ColorSpace;
use zune_image::core_filters::colorspace::ColorspaceConv;
use zune_image::image::Image;
use zune_image::metadata::ImageMetadata;

fn main() {
    let mut image = Image::new(vec![], ImageMetadata::default().depth(), 0usize, 0usize, ColorSpace::BGR);
    let _ = <zune_image::core_filters::colorspace::ColorspaceConv as zune_image::traits::OperationsTrait>::execute_impl(&ColorspaceConv::new(ColorSpace::Unknown), &mut image);
}
