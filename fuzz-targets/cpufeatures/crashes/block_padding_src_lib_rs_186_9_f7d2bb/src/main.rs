fn main() {
    <block_padding::Pkcs7 as block_padding::Padding>::raw_pad(&mut include_bytes!("data_0.bin").to_vec(), 0usize);
}
