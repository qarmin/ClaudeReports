use lzma_rust2::EncodeMode;
use lzma_rust2::LzmaOptions;
use lzma_rust2::MfType;

fn main() {
    let lzma_options = LzmaOptions { dict_size: 0u32, lc: u32::MAX, lp: 0u32, pb: 1u32, mode: EncodeMode::Normal, nice_len: 0u32, mf: MfType::Hc4, depth_limit: 0i32, preset_dict: None };
    let _ = lzma_options.get_props();
}
