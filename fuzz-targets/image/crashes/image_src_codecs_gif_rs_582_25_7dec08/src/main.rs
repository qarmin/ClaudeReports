use image::Frame;
use image::codecs::bmp::RowsDecoded;

fn main() {
    let rows_decoded = RowsDecoded::TopDown { rows: 0u32 };
    let u32_v = rows_decoded.rows();
    let mut gif_encoder = <image::codecs::gif::GifEncoder<Vec<u8>>>::new(vec![]);
    let _ = gif_encoder.encode_frame(Frame::new(<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>::from_raw(u32_v, 1u32, vec![71u8]).unwrap()));
}
