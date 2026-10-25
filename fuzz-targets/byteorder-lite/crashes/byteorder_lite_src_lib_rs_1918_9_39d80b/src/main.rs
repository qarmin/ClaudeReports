fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_u16_into(&[], &mut vec![45u8]);
}
