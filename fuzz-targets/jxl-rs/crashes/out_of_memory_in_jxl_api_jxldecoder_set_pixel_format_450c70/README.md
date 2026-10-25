# out of memory in jxl::api::JxlDecoder::set_pixel_format

- crate: https://github.com/libjxl/jxl-rs @ 7f539c803ddd9c5d2eb654de0814d8595e2225b2
- kind: out of memory
- last API call: `jxl::api::JxlDecoder::set_pixel_format`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling jxl::api::JxlDecoder::set_pixel_format
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jxl = { git = "https://github.com/libjxl/jxl-rs", rev = "7f539c803ddd9c5d2eb654de0814d8595e2225b2" }
```

`src/main.rs`:

```rust
use jxl::api::JxlDecoder;
use jxl::api::JxlDecoderOptions;
use jxl::api::JxlPixelFormat;

fn main() {
    let mut jxl_decoder = JxlDecoder::new(JxlDecoderOptions::default());
    let _ = jxl_decoder.set_pixel_format(JxlPixelFormat::rgba_f16(1000000000000000000usize));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_jxl_api_jxldecoder_set_pixel_format_450c70`).

## Output

```
memory allocation of 2000000000000000000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<core::option::Option<jxl::api::data_types::JxlDataFormat>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<core::option::Option<jxl::api::data_types::JxlDataFormat>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <core::option::Option<jxl::api::data_types::JxlDataFormat> as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:14:21
   9: alloc::vec::from_elem::<core::option::Option<jxl::api::data_types::JxlDataFormat>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
  10: <jxl::api::data_types::JxlPixelFormat>::rgba_f16
             at /home/rafal/.cargo/git/checkouts/jxl-rs-9a2d2706c52dfcc5/7f539c8/jxl/src/api/data_types.rs:211:35
  11: repro_out_of_memory_in_jxl_api_jxldecoder_set_pixel_format_450c70::main
             at ./src/main.rs:7:42
  12: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
