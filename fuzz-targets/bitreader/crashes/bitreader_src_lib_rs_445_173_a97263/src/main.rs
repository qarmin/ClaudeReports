use bitreader::BitReader;
use bitreader::BitReaderError;

fn main() {
    let vec = vec![];
    let bit_reader = BitReader::new(&vec);
    let u64_v = bit_reader.position();
    let bit_reader_error = BitReaderError::NotEnoughData { position: 1u64, length: u64_v, requested: 0u64 };
    let _ = bit_reader_error.to_string();
}
