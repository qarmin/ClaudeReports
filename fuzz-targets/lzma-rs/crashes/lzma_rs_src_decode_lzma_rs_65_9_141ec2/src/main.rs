use lzma_rs::decompress::raw::LzmaDecoder;
use lzma_rs::decompress::raw::LzmaParams;
use lzma_rs::decompress::raw::LzmaProperties;

fn main() {
    let _ = LzmaDecoder::new(LzmaParams::new(LzmaProperties { lc: 0u32, lp: 1u32, pb: 100u32 }, 0u32, None), None);
}
