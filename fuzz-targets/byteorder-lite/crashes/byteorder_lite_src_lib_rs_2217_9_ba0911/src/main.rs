fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_i128_into(&[], &mut vec![0u8]);
}
