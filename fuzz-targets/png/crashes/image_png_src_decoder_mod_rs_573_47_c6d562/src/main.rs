use png::DecodeOptions;

fn main() {
    let decoder = <png::Decoder<::std::io::Cursor<Vec<u8>>>>::new_with_options(std::io::Cursor::new(include_bytes!("data_0.bin").to_vec()), DecodeOptions::default());
    let mut reader = decoder.read_info().unwrap();
    let _ = reader.read_row(&mut vec![]);
}
