fn main() {
    let u128 = <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_u128(&vec![0u8, 0u8, 0u8, 0u8, 79u8, 0u8, 0u8, 0u8, 35u8, 0u8, 0u8, 0u8, 0u8, 0u8, 69u8, 101u8]);
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_uint128(&mut vec![], u128, 0usize);
}
