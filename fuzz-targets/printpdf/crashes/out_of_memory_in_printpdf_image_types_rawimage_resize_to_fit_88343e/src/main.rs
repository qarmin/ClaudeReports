use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(1usize, 4294967296usize, RawImageFormat::R8);
    let _ = raw_image.resize_to_fit_size(2147483648usize);
}
