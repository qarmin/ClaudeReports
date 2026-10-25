fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_u32_into(&[], &mut vec![0u8]);
}
