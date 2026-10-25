use lzma_rust2::EncodeMode;
use lzma_rust2::LzmaOptions;
use lzma_rust2::MfType;

fn main() {
    let mf_type = MfType::Bt4;
    let lzma_options = LzmaOptions::new(1u32, 0u32, 1000000000u32, 0u32, EncodeMode::Fast, 0u32, mf_type, 0i32);
    let _ = lzma_options.get_props();
}
