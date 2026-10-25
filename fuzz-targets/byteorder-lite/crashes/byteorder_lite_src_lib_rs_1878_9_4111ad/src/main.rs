fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_u64_into(&vec![116u8], &mut vec![]);
}
