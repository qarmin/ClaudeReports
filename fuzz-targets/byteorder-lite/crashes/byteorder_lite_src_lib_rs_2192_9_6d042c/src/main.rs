fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_u64_into(&[], &mut vec![84u8]);
}
