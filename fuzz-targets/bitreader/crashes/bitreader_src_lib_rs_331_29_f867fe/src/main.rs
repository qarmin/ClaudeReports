use bitreader::BitReader;

fn main() {
    let vec = vec![];
    let mut bit_reader = BitReader::new(&vec);
    let _ = bit_reader.align(0u32);
}
