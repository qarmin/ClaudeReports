fn main() {
    let u128 = <byteorder::BigEndian as byteorder::ByteOrder>::read_u128(&vec![73u8, 73u8, 42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
    <byteorder::BigEndian as byteorder::ByteOrder>::write_uint128(&mut vec![], u128, 0usize);
}
