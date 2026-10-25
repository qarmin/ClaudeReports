fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_u128_into(&[], &mut vec![82u8]);
}
