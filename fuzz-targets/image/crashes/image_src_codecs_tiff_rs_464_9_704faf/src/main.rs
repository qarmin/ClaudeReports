fn main() {
    let mut tiff_decoder = <image::codecs::tiff::TiffDecoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(include_bytes!("data_0.bin").to_vec())).unwrap();
    let _ = <image::codecs::tiff::TiffDecoder<::std::io::Cursor<Vec<u8>>> as image::ImageDecoder>::read_image(&mut tiff_decoder, &mut vec![]);
}
