use jxl_image::color::EnumColourEncoding;
use jxl_image::color::RenderingIntent;

fn main() {
    let _ = jxl_color::icc::colour_encoding_to_icc(&EnumColourEncoding::xyb(RenderingIntent::Perceptual));
}
