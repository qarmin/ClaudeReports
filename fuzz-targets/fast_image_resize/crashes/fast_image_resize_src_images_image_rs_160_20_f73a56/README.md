# fast_image_resize/src/images/image.rs:160:20

- crate: fast_image_resize 6.1.0
- kind: panic
- location: `fast_image_resize/src/images/image.rs:160:20`
- last API call: `fast_image_resize::images::Image::from_slice_u8`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17449254768289490682

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fast_image_resize = { version = "=6.1.0" }
```

`src/main.rs`:

```rust
use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let _ = Image::from_slice_u8(u32::MAX, 1000000000u32, &mut vec![], PixelType::F32x2);
}
```

Run with `cargo run` in this directory (package `repro_fast_image_resize_src_images_image_rs_160_20_f73a56`).

## Output

```
thread 'main' (1070198) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fast_image_resize-6.1.0/src/images/image.rs:160:20:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <fast_image_resize::images::image::Image>::from_slice_u8
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fast_image_resize-6.1.0/src/images/image.rs:160:20
   4: repro_fast_image_resize_src_images_image_rs_160_20_f73a56::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
