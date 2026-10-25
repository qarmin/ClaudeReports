use printpdf::image_types::ImageOptimizationOptions;
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(1000000000000000000usize, 10000usize, RawImageFormat::R8);
    let _ = raw_image.optimize(&ImageOptimizationOptions::default());
}
