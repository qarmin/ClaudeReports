fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_u64_into(&[], &mut vec![0u8]);
}
