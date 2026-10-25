fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_u64_into(&vec![], &mut vec![0u64]);
}
