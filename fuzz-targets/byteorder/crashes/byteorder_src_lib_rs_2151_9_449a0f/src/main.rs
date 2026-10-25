fn main() {
    <byteorder::LittleEndian as byteorder::ByteOrder>::read_u16_into(&vec![0u8], &mut vec![]);
}
