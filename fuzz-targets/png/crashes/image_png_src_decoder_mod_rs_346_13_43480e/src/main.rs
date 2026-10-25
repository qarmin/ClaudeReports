fn main() {
    let decoder = <png::Decoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(include_bytes!("data_0.bin").to_vec()));
    let mut reader = decoder.read_info().unwrap();
    let _ = reader.finish();
    let _ = reader.next_frame_info();
}
