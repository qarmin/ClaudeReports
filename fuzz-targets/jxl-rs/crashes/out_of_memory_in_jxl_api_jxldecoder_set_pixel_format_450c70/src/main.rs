use jxl::api::JxlDecoder;
use jxl::api::JxlDecoderOptions;
use jxl::api::JxlPixelFormat;

fn main() {
    let mut jxl_decoder = JxlDecoder::new(JxlDecoderOptions::default());
    let _ = jxl_decoder.set_pixel_format(JxlPixelFormat::rgba_f16(1000000000000000000usize));
}
