fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_u128_into(&[], &mut vec![60u8]);
}
