use ruzstd::decoding::DEFAULT_MAX_WINDOW_SIZE;
use ruzstd::decoding::errors::BlockSizeError;
use ruzstd::decoding::errors::FSEDecoderError;
use ruzstd::encoding::CompressionLevel;
use ruzstd::fse::FSETable;
use ruzstd::huff0::huff0_encoder::HuffmanTable;

fn main() {
    let mut f_s_e_table = FSETable::new(64u8);
    let _ = f_s_e_table.build_from_probabilities(26u8, &[7i32, 2041339626i32, 45i32, i32::MAX, 1i32, 1i32]);
    let _ = HuffmanTable::build_from_weights(&[3usize, 6180167714942147986usize, 2usize]);
    let _ = BlockSizeError::BlockSizeTooLarge { size: 0u32 };
    let _ = DEFAULT_MAX_WINDOW_SIZE;
    let _ = CompressionLevel::Default;
    let _ = FSEDecoderError::TableIsUninitialized;
}
