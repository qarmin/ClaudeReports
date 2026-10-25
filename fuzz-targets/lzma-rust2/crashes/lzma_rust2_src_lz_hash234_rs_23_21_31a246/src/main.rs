use lzma_rust2::EncodeMode;
use lzma_rust2::LzmaOptions;
use lzma_rust2::MfType;
use lzma_rust2::filter::bcj2::Bcj2Options;

fn main() {
    let u32_v = lzma_rust2::get_extra_size_before(1000000000u32);
    let u32_v2 = Bcj2Options::RELATIVE_LIMIT_DEFAULT;
    let lzma_options = LzmaOptions::new(u32_v, 0u32, 0u32, u32_v2, EncodeMode::Normal, 0u32, MfType::Hc4, 0i32);
    let _ = lzma_options.get_memory_usage();
}
