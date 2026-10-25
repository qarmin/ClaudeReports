fn main() {
    let _ = lz4_flex::decompress_size_prepended(&vec![82u8, 184u8, 46u8, 226u8]);
}
