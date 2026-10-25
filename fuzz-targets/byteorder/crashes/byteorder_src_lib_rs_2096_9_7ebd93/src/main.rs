fn main() {
    let _ = <byteorder::LittleEndian as byteorder::ByteOrder>::read_uint(&vec![], 0usize);
}
