fn main() {
    <byteorder::LittleEndian as byteorder::ByteOrder>::write_u16_into(&[], &mut vec![0u8]);
}
