use lzma_rust2::EncodeMode;
use lzma_rust2::LzmaOptions;
use lzma_rust2::MfType;

fn main() {
    let lzma_options = LzmaOptions { dict_size: 4294967294u32, lc: 0u32, lp: 0u32, pb: 0u32, mode: EncodeMode::Fast, nice_len: 1u32, mf: MfType::default(), depth_limit: 0i32, preset_dict: None };
    let _ = lzma_options.get_memory_usage();
}
