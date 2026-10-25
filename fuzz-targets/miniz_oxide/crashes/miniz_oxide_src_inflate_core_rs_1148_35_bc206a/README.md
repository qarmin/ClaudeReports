# miniz_oxide/src/inflate/core.rs:1148:35

- crate: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf
- kind: panic
- location: `miniz_oxide/src/inflate/core.rs:1148:35`
- last API call: `miniz_oxide::inflate::core::decompress`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 1 but the index is 18446744073709551597
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
miniz_oxide = { git = "https://github.com/Frommi/miniz_oxide", rev = "cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf", features = ["alloc", "block-boundary", "core", "serde", "simd", "simd-adler32", "std", "with-alloc"] }
```

`src/main.rs`:

```rust
use miniz_oxide::inflate::core::BlockBoundaryState;
use miniz_oxide::inflate::core::DecompressorOxide;
use miniz_oxide::inflate::core::inflate_flags::TINFL_FLAG_PARSE_ZLIB_HEADER;

fn main() {
    let u32_v = TINFL_FLAG_PARSE_ZLIB_HEADER;
    let block_boundary_state = BlockBoundaryState::default();
    let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(&block_boundary_state);
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &vec![35u8, 32u8, 84u8], &mut vec![0u8, 0u8, 0u8, 0u8, 91u8, 76u8, 0u8, 0u8, 42u8, 39u8, 0u8, 0u8, 98u8, 51u8, 0u8, 0u8, 0u8, 0u8, 0u8, 58u8, 49u8, 0u8, 0u8, 0u8, 61u8, 39u8, 33u8, 0u8, 0u8, 111u8, 0u8, 69u8], 2usize, u32_v);
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &vec![], &mut vec![0u8], 0usize, u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_miniz_oxide_src_inflate_core_rs_1148_35_bc206a`).

## Output

```
thread 'main' (1273914) panicked at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1148:35:
index out of bounds: the len is 1 but the index is 18446744073709551597
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: miniz_oxide::inflate::core::transfer
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1148:35
   4: miniz_oxide::inflate::core::decompress_with_limit
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1966:21
   5: miniz_oxide::inflate::core::decompress
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1420:5
   6: repro_miniz_oxide_src_inflate_core_rs_1148_35_bc206a::main
             at ./src/main.rs:10:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
