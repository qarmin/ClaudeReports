# image/src/codecs/gif.rs:524:33

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/gif.rs:524:33`
- last API call: `image::codecs::gif::GifEncoder<Vec<u8>>::encode`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 11367997957251169362

## Message

```
assertion `left == right` failed: Too many or too little pixels for the given width and height to create a GIF Frame
  left: 1
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
use image::ExtendedColorType;

fn main() {
    let mut gif_encoder = <image::codecs::gif::GifEncoder<Vec<u8>>>::new(vec![]);
    let _ = gif_encoder.encode(&vec![], 1u32, 1u32, ExtendedColorType::L8);
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_gif_rs_524_33_44b37f`).

## Output

```
thread 'main' (345810) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:524:33:
assertion `left == right` failed: Too many or too little pixels for the given width and height to create a GIF Frame
  left: 1
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
   4: <gif::common::Frame>::from_palette_pixels::<&[u8], alloc::vec::Vec<u8>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gif-0.14.2/src/common.rs:394:9
   5: <image::codecs::gif::GifEncoder<alloc::vec::Vec<u8>>>::encode
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:524:33
   6: repro_image_src_codecs_gif_rs_524_33_44b37f::main
             at ./src/main.rs:5:25
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
