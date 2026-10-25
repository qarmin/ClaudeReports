fn main() {
    let _ = <block_padding::Pkcs7 as block_padding::Padding>::raw_unpad(&include_bytes!("data_0.bin").to_vec());
}
