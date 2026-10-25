# gif/src/common.rs:299:9

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `gif/src/common.rs:299:9`
- source: https://docs.rs/crate/gif/0.14.2/source/src/common.rs#299
- last API call: `image::codecs::gif::GifEncoder<Vec<u8>>::encode`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15370919622368298418

## Message

```
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 2
 right: 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::ColorType;
use image::ExtendedColorType;

fn main() {
    let mut gif_encoder = <image::codecs::gif::GifEncoder<Vec<u8>>>::new(vec![]);
    let _ = gif_encoder.encode(&vec![], 1u32, 1u32, ExtendedColorType::from(ColorType::La8));
}
```

Run with `cargo run` in this directory (package `repro_gif_src_common_rs_299_9_7a5082`).

## Output

```
thread 'main' (351065) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gif-0.14.2/src/common.rs:299:9:
assertion `left == right` failed: Too much or too little pixel data for the given width and height to create a GIF Frame
  left: 2
 right: 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:447:23
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <gif::common::Frame>::from_grayscale_with_alpha
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gif-0.14.2/src/common.rs:299:9
   5: <image::codecs::gif::GifEncoder<alloc::vec::Vec<u8>>>::encode
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:529:33
   6: repro_gif_src_common_rs_299_9_7a5082::main
             at ./src/main.rs:6:25
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
