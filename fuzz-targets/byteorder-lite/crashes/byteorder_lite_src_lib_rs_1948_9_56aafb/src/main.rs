fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_i64_into(&[], &mut vec![0u8]);
}
