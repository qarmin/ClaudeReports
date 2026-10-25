#![no_main]
use jxl_fuzz_formats::{FuzzConfig, fuzz_decode};
use libfuzzer_sys::fuzz_target;

// Run the binary under `ulimit -v`: an allocation failure must surface as an Err, never an abort.
fuzz_target!(|data: &[u8]| {
    let _ = fuzz_decode(data, FuzzConfig { sample_limit: None, ..Default::default() });
});
