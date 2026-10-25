fn main() {
    <byteorder_lite::LittleEndian as byteorder_lite::ByteOrder>::write_f64_into(&[], &mut vec![60u8]);
}
