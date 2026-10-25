fn main() {
    let encode_builder = bs58::encode::<Vec<u8>>(include_bytes!("data_0.bin").to_vec());
    let encode_builder2 = encode_builder.as_cb58(Some(0u8));
    let _ = encode_builder2.into_vec();
}
