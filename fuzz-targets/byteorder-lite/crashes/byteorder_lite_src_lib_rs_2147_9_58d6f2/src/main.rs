fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_u128_into(&vec![82u8], &mut vec![]);
}
