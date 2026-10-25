fn main() {
    <byteorder::BigEndian as byteorder::ByteOrder>::read_u16_into(&vec![], &mut vec![0u16]);
}
