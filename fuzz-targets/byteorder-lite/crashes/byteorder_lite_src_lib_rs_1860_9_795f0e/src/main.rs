fn main() {
    let u128 = <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_u128(&vec![58u8, 0u8, 0u8, 0u8, 108u8, 67u8, 0u8, 0u8, 60u8, 92u8, 0u8, 0u8, 0u8, 76u8, 0u8, 0u8]);
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_uint128(&mut vec![], u128, 0usize);
}
