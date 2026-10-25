use printpdf::image_types::ImageOptimizationOptions;
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(0usize, 1000000000usize, RawImageFormat::RG8);
    let _ = raw_image.optimize(&ImageOptimizationOptions { quality: None, max_image_size: None, dither_greyscale: Some(true), convert_to_greyscale: None, auto_optimize: None, format: None });
}
