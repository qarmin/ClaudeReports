use printpdf::image_types::ImageCompression;
use printpdf::image_types::ImageOptimizationOptions;
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(10000000000000000000usize, 5usize, RawImageFormat::RG8);
    let _ = raw_image.optimize(&ImageOptimizationOptions { quality: Some(1.1920929e-7f32), max_image_size: Some("Zażółć gęślą jaźń".to_string()), dither_greyscale: Some(true), convert_to_greyscale: Some(true), auto_optimize: Some(true), format: Some(ImageCompression::Flate) });
}
