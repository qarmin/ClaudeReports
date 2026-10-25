fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::read_i64_into(&vec![66u8], &mut vec![]);
}
