# zune-image/src/image.rs:707:10

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/image.rs:707:10`
- last API call: `zune_image::core_filters::depth::Depth::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 75104399752825117

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false, features = ["bmp", "farbfeld", "hdr", "heic", "jpeg", "jpeg-xl", "jpeg_2000", "png", "ppm", "psd", "qoi", "webp", "metadata", "serde-support", "threads", "simd", "log"] }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::colorspace::ColorSpace;
use zune_image::codecs::jpeg::JpegEncoder;
use zune_image::core_filters::depth::Depth;
use zune_image::image::Image;

fn main() {
    let color_space = <zune_image::codecs::jpeg::JpegEncoder as zune_image::traits::EncoderTrait>::default_colorspace(&JpegEncoder::default(), ColorSpace::BGRA);
    let _ = Depth::new(Image::from_f32(&[], usize::MAX, 2147483648usize, color_space).depth());
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_image_rs_707_10_5c2fe1`).

## Output

```
thread 'main' (2359121) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:707:10:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<usize>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: zune_image::image::checked_mul
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:707:10
   6: <zune_image::image::Image>::from_f32
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:543:28
   7: repro_zune_image_src_image_rs_707_10_5c2fe1::main
             at ./src/main.rs:8:24
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
