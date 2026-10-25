use bitreader::BitReader;

fn main() {
    let vec = vec![0u8, 0u8, 74u8, 70u8, 73u8];
    let bit_reader = BitReader::new(&vec);
    let mut bit_reader2 = bit_reader.relative_reader_atmost(u64::MAX);
    let _ = bit_reader2.read_i64(38u8);
    let _ = bit_reader2.skip(18446744073709551614u64);
}
