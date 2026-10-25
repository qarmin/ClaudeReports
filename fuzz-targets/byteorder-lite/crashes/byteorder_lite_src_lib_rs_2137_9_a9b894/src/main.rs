fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_u32_into(&vec![], &mut vec![0u32]);
}
