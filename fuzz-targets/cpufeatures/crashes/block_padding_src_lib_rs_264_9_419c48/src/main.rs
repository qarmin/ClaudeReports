fn main() {
    let _ = <block_padding::AnsiX923 as block_padding::Padding>::raw_unpad(&include_bytes!("data_0.bin").to_vec());
}
