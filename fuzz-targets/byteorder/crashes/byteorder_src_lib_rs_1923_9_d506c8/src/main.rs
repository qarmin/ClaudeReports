fn main() {
    let _ = <byteorder::BigEndian as byteorder::ByteOrder>::read_uint128(&vec![], 0usize);
}
