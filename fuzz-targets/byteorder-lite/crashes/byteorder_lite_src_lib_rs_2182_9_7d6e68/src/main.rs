fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_u16_into(&[], &mut vec![0u8]);
}
