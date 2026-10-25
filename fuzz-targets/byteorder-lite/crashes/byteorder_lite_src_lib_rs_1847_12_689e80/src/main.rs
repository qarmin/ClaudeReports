fn main() {
    let u128 = <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_u128(&vec![0u8, 80u8, 78u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 73u8, 72u8, 68u8, 82u8]);
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::write_u128(&mut vec![], u128);
}
