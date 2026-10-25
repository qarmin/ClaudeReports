fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_u16_into(&vec![0u8], &mut vec![]);
}
