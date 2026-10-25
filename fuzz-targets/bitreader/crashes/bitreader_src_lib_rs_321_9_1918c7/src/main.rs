use bitreader::BitReader;

fn main() {
    let vec = vec![];
    let bit_reader = BitReader::new(&vec);
    let _ = bit_reader.is_aligned(0u32);
}
