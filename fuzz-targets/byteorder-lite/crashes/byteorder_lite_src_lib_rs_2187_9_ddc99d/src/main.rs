fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_u32_into(&[], &mut vec![73u8]);
}
