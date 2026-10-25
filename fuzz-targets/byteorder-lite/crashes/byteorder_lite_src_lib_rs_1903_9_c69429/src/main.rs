fn main() {
    <byteorder_lite::BigEndian as byteorder_lite::ByteOrder>::read_i128_into(&vec![45u8], &mut vec![]);
}
