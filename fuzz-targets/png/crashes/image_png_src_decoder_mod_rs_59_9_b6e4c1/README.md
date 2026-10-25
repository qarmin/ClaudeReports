# image-png/src/decoder/mod.rs:59:9

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/decoder/mod.rs:59:9`
- last API call: `png::OutputInfo::buffer_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
png = { git = "https://github.com/image-rs/image-png", rev = "b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc" }
```

`src/main.rs`:

```rust
use png::BitDepth;
use png::ColorType;
use png::OutputInfo;

fn main() {
    let output_info = OutputInfo { width: 0u32, height: 100000000u32, color_type: ColorType::Rgba, bit_depth: BitDepth::from_u8(1u8).unwrap(), line_size: 10000000000000000000usize };
    let _ = output_info.buffer_size();
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_decoder_mod_rs_59_9_b6e4c1`).

## Output

```
thread 'main' (1263096) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/decoder/mod.rs:59:9:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <png::decoder::OutputInfo>::buffer_size
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/decoder/mod.rs:59:9
   4: repro_image_png_src_decoder_mod_rs_59_9_b6e4c1::main
             at ./src/main.rs:7:25
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
