fn main() {
    let _ = <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_uint128(&vec![], 0usize);
}
