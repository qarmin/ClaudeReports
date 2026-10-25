fn main() {
    let encode_builder = bs58::encode::<Vec<u8>>(include_bytes!("data_0.bin").to_vec());
    let _ = encode_builder.onto("(\\)".to_string());
}
