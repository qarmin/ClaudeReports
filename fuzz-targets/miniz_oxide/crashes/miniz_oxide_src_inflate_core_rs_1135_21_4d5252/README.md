# miniz_oxide/src/inflate/core.rs:1135:21

- crate: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf
- kind: panic
- location: `miniz_oxide/src/inflate/core.rs:1135:21`
- last API call: `miniz_oxide::inflate::core::decompress_with_limit`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15177260802197862138

## Message

```
attempt to add with overflow
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
    let _ = { let r = &mut decompressor_oxide; let in_buf = &vec![82u8, 73u8, 70u8, 70u8]; let out = &mut vec![0u8, 0u8, 0u8, 0u8]; let out_pos = 0usize; let flags = 0u32; miniz_oxide::inflate::core::decompress(r, in_buf, out, out_pos, flags).0 };
    let _ = miniz_oxide::inflate::core::decompress_with_limit(&mut decompressor_oxide, &vec![], &mut vec![35u8, 32u8, 84u8, 72u8, 73u8], 1usize, 5usize, u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_miniz_oxide_src_inflate_core_rs_1135_21_4d5252`).

## Output

```
thread 'main' (329510) panicked at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1135:21:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: miniz_oxide::inflate::core::transfer
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1135:21
   4: miniz_oxide::inflate::core::decompress_with_limit
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1966:21
   5: repro_miniz_oxide_src_inflate_core_rs_1135_21_4d5252::main
             at ./src/main.rs:7:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
