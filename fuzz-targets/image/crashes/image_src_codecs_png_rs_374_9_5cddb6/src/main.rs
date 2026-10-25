use image::Limits;

fn main() {
    let mut png_decoder = <image::codecs::png::PngDecoder<::std::io::Cursor<Vec<u8>>>>::with_limits(std::io::Cursor::new(include_bytes!("data_0.bin").to_vec()), Limits::no_limits());
    let _ = <image::codecs::png::PngDecoder<::std::io::Cursor<Vec<u8>>> as image::ImageDecoder>::read_image(&mut png_decoder, &mut vec![]);
}
