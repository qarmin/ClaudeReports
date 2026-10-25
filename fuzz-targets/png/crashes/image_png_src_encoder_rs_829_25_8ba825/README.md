# image-png/src/encoder.rs:829:25

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/encoder.rs:829:25`
- last API call: `png::Writer<Vec<u8>>::write_image_data`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14023440431276622490

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
use png::ColorType;
use png::Info;

fn main() {
    let mut encoder = <png::Encoder<Vec<u8>>>::with_info(vec![], Info::with_size(u32::MAX, 1874738817u32)).unwrap();
    encoder.set_color(ColorType::Rgb);
    let mut writer = encoder.write_header().unwrap();
    let _ = writer.write_image_data(&vec![]);
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_encoder_rs_829_25_8ba825`).

## Output

```
thread 'main' (775426) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/encoder.rs:829:25:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <png::encoder::Writer<alloc::vec::Vec<u8>>>::write_image_data
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/encoder.rs:829:25
   4: repro_image_png_src_encoder_rs_829_25_8ba825::main
             at ./src/main.rs:8:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
