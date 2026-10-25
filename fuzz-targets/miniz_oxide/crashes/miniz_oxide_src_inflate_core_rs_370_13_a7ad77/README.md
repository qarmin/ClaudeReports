# miniz_oxide/src/inflate/core.rs:370:13

- crate: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf
- kind: panic
- location: `miniz_oxide/src/inflate/core.rs:370:13`
- last API call: `miniz_oxide::inflate::core::DecompressorOxide::block_boundary_state`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: self.num_bits < 8
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
    let decompressor_oxide = DecompressorOxide::from_block_boundary_state(&BlockBoundaryState { num_bits: 100u8, bit_buf: 0u8, z_header0: 1u32, z_header1: 0u32, check_adler32: 1u32 });
    let _ = decompressor_oxide.block_boundary_state();
}
```

Run with `cargo run` in this directory (package `repro_miniz_oxide_src_inflate_core_rs_370_13_a7ad77`).

## Output

```
thread 'main' (1273345) panicked at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:370:13:
assertion failed: self.num_bits < 8
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <miniz_oxide::inflate::core::DecompressorOxide>::block_boundary_state
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:370:13
   4: repro_miniz_oxide_src_inflate_core_rs_370_13_a7ad77::main
             at ./src/main.rs:6:32
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
