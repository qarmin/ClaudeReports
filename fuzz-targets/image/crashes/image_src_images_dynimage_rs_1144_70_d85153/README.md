# image/src/images/dynimage.rs:1144:70

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/images/dynimage.rs:1144:70`
- last API call: `image::DynamicImage::resize_to_fill`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: "Image size must not be zero"
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::DynamicImage;
use image::imageops::FilterType;

fn main() {
    let mut dynamic_image = DynamicImage::from(DynamicImage::new_luma16(0u32, 0u32));
    dynamic_image.resize_to_fill(0u32, 1u32, FilterType::Triangle);
}
```

Run with `cargo run` in this directory (package `repro_image_src_images_dynimage_rs_1144_70_d85153`).

## Output

```
thread 'main' (3295574) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1144:70:
called `Result::unwrap()` on an `Err` value: "Image size must not be zero"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(), alloc::string::String>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <image::images::dynimage::DynamicImage>::resize_exact
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1144:70
   5: <image::images::dynimage::DynamicImage>::resize_to_fill
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/dynimage.rs:1191:14
   6: repro_image_src_images_dynimage_rs_1144_70_d85153::main
             at ./src/main.rs:6:19
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
