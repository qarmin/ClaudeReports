use rawler::bits::Endian;

fn main() {
    let endian = Endian::default();
    let _ = endian.read_i8(&vec![], 0usize);
}
