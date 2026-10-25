use printpdf::image_types::RawImage;
use printpdf::image_types::RawImageFormat;

fn main() {
    let mut raw_image = RawImage::empty(10000000000000000000usize, 65536usize, RawImageFormat::R16);
    let _ = raw_image.apply_dithering();
}
