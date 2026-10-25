fn main() {
    let _ = <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_uint128(&vec![], 0usize);
}
