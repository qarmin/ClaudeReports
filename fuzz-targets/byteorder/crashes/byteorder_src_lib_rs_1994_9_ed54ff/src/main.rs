fn main() {
    <byteorder::BigEndian as byteorder::ByteOrder>::read_u128_into(&vec![66u8], &mut vec![]);
}
