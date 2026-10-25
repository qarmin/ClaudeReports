# miniz_oxide/src/deflate/core.rs:1985:64

- crate: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf
- kind: panic
- location: `miniz_oxide/src/deflate/core.rs:1985:64`
- last API call: `miniz_oxide::deflate::core::compress`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
miniz_oxide = { git = "https://github.com/Frommi/miniz_oxide", rev = "cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf" }
```

`src/main.rs`:

```rust
use miniz_oxide::DataFormat;
use miniz_oxide::MZFlush;
use miniz_oxide::deflate::CompressionLevel;
use miniz_oxide::deflate::core::CompressionStrategy;
use miniz_oxide::deflate::core::CompressorOxide;
use miniz_oxide::deflate::core::TDEFLFlush;

fn main() {
    let mut compressor_oxide = CompressorOxide::with_params(CompressorOxide::with_params(DataFormat::Raw, 0u8, CompressionStrategy::Filtered, 0u8).data_format(), 1u8, CompressionStrategy::Filtered, 0u8);
    let _ = miniz_oxide::deflate::stream::deflate(&mut compressor_oxide, &include_bytes!("data_0.bin").to_vec(), &mut vec![0u8], MZFlush::Block);
    compressor_oxide.set_compression_level(CompressionLevel::UberCompression);
    let _ = miniz_oxide::deflate::core::compress(&mut compressor_oxide, &vec![66u8], &mut vec![], TDEFLFlush::None);
}
```

Run with `cargo run` in this directory (package `repro_miniz_oxide_src_deflate_core_rs_1985_64_7cacb5`).

## Output

```
thread 'main' (1273571) panicked at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:1985:64:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: miniz_oxide::deflate::core::compress_normal
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:1985:64
   4: miniz_oxide::deflate::core::compress_inner
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:2483:9
   5: miniz_oxide::deflate::core::compress
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:2414:5
   6: repro_miniz_oxide_src_deflate_core_rs_1985_64_7cacb5::main
             at ./src/main.rs:12:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
