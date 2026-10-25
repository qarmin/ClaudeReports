fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_f32_into(&[], &mut vec![0u8]);
}
