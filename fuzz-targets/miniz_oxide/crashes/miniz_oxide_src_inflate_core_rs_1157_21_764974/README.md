# miniz_oxide/src/inflate/core.rs:1157:21

- crate: https://github.com/Frommi/miniz_oxide @ cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf
- kind: panic
- location: `miniz_oxide/src/inflate/core.rs:1157:21`
- last API call: `miniz_oxide::inflate::core::decompress`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
miniz_oxide = { git = "https://github.com/Frommi/miniz_oxide", rev = "cdd7ddd56cf57a6255203bb9f5c6a92bf1144bbf" }
```

`src/main.rs`:

```rust
use miniz_oxide::inflate::core::DecompressorOxide;

fn main() {
    let mut decompressor_oxide = DecompressorOxide::new();
    let _ = miniz_oxide::inflate::core::decompress_with_limit(&mut decompressor_oxide, &vec![82u8, 73u8, 70u8, 70u8], &mut vec![0u8, 0u8, 0u8, 0u8, 60u8, 0u8, 86u8, 123u8, 71u8, 66u8, 0u8, 0u8, 0u8, 33u8], 3usize, 10000000000000000000usize, 2388820830u32);
    let _ = miniz_oxide::inflate::core::decompress(&mut decompressor_oxide, &include_bytes!("image_0.bmp").to_vec(), &mut include_bytes!("image_1.jpg").to_vec(), 2usize, 968484430u32);
}
```

Run with `cargo run` in this directory (package `repro_miniz_oxide_src_inflate_core_rs_1157_21_764974`).

## Output

```
thread 'main' (1274008) panicked at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1157:21:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: miniz_oxide::inflate::core::transfer
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1157:21
   4: miniz_oxide::inflate::core::decompress_with_limit
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1966:21
   5: miniz_oxide::inflate::core::decompress
             at /home/rafal/.cargo/git/checkouts/miniz_oxide-e6175b69d2bbf328/cdd7ddd/miniz_oxide/src/inflate/core.rs:1420:5
   6: repro_miniz_oxide_src_inflate_core_rs_1157_21_764974::main
             at ./src/main.rs:6:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
