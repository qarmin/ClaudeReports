use snap::raw::Decoder;

fn main() {
    let mut decoder = Decoder::new();
    let _ = decoder.decompress_vec(&vec![144u8, 216u8, 179u8, 191u8, 11u8]);
}
