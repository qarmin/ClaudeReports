# miniz_oxide/src/inflate/core.rs:1151:13

- crate: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf
- kind: panic
- location: `miniz_oxide/src/inflate/core.rs:1151:13`
- last API call: `miniz_oxide::inflate::core::decompress_with_limit`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: (source_pos + 1) & out_buf_size_mask < out_slice.len()
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

fn main() {
    let mut decompressor_oxide = DecompressorOxide::from_block_boundary_state(&BlockBoundaryState::default());
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &vec![35u8, 32u8, 84u8], &mut vec![0u8, 68u8, 78u8, 0u8, 66u8, 0u8, 126u8, 0u8, 0u8, 32u8, 0u8, 0u8, 0u8, 0u8, 0u8, 77u8, 0u8, 0u8, 75u8, 0u8, 0u8, 0u8, 104u8, 76u8, 0u8, 113u8, 0u8, 60u8, 99u8, 82u8, 42u8, 0u8], 3usize, 1u32);
    let _ = miniz_oxide::inflate::core::decompress_with_limit(&mut decompressor_oxide, &vec![], &mut vec![82u8, 73u8], 0usize, 1000000000usize, 6u32);
}
```

Run with `cargo run` in this directory (package `repro_miniz_oxide_src_inflate_core_rs_1151_13_22cc58`).

## Output

```
thread 'main' (1273763) panicked at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1151:13:
assertion failed: (source_pos + 1) & out_buf_size_mask < out_slice.len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: miniz_oxide::inflate::core::transfer
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1151:13
   4: miniz_oxide::inflate::core::decompress_with_limit
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1966:21
   5: repro_miniz_oxide_src_inflate_core_rs_1151_13_22cc58::main
             at ./src/main.rs:7:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
