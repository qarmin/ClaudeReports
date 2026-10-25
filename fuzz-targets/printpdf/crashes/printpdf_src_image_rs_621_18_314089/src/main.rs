use printpdf::image_types::ImageCompression;
use printpdf::image_types::ImageOptimizationOptions;
use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(usize::MAX, 15usize, RawImageFormat::R8);
    let _ = raw_image.optimize(&ImageOptimizationOptions { quality: Some(0.9999999f32), max_image_size: Some("😀👍🏽".to_string()), dither_greyscale: Some(true), convert_to_greyscale: None, auto_optimize: Some(false), format: Some(ImageCompression::Auto) });
}
