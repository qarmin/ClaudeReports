use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(0usize, 1000000000usize, RawImageFormat::RG8);
    let _ = raw_image.apply_dithering();
}
