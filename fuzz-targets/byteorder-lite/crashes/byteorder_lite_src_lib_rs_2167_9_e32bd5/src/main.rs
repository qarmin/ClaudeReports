fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_i128_into(&vec![0u8], &mut vec![]);
}
