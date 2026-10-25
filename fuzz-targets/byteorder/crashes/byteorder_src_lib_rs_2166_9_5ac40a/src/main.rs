fn main() {
    <byteorder::LittleEndian as byteorder::ByteOrder>::read_u128_into(&vec![123u8], &mut vec![]);
}
