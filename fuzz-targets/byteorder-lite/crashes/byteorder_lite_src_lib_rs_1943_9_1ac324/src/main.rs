fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_i32_into(&[], &mut vec![0u8]);
}
