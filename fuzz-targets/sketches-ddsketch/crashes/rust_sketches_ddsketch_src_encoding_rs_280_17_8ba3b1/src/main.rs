fn main() {
    let _ = sketches_ddsketch::encoding::decode_from_java_bytes(&include_bytes!("data_0.bin").to_vec());
}
