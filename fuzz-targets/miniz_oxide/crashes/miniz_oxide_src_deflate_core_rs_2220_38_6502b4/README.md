# miniz_oxide/src/deflate/core.rs:2220:38

- crate: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf
- kind: panic
- location: `miniz_oxide/src/deflate/core.rs:2220:38`
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
use miniz_oxide::deflate::core::CompressorOxide;
use miniz_oxide::deflate::core::TDEFLFlush;

fn main() {
    let mut compressor_oxide = CompressorOxide::default();
    let _ = miniz_oxide::deflate::stream::deflate(&mut compressor_oxide, &vec![0u8, 0u8, 0u8, 0u8], &mut vec![0u8], MZFlush::Block);
    compressor_oxide.set_format_and_level(CompressorOxide::with_format_and_level(DataFormat::Zlib, CompressionLevel::BestSpeed).data_format(), 1u8);
    let _ = miniz_oxide::deflate::core::compress(&mut compressor_oxide, &vec![], &mut vec![], TDEFLFlush::NoSync);
}
```

Run with `cargo run` in this directory (package `repro_miniz_oxide_src_deflate_core_rs_2220_38_6502b4`).

## Output

```
thread 'main' (1273840) panicked at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:2220:38:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: miniz_oxide::deflate::core::compress_fast
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:2220:38
   4: miniz_oxide::deflate::core::compress_inner
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:2481:9
   5: miniz_oxide::deflate::core::compress
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/deflate/core.rs:2414:5
   6: repro_miniz_oxide_src_deflate_core_rs_2220_38_6502b4::main
             at ./src/main.rs:11:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
