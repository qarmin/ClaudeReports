# image/src/codecs/png.rs:374:9

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/png.rs:374:9`
- last API call: `image::codecs::png::PngDecoder<::std::io::Cursor<Vec<u8>>>::read_image`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17288415074151697140

## Message

```
assertion `left == right` failed
  left: Ok(0)
 right: Ok(1)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::Limits;

fn main() {
    let mut png_decoder = <image::codecs::png::PngDecoder<::std::io::Cursor<Vec<u8>>>>::with_limits(std::io::Cursor::new(include_bytes!("data_0.bin").to_vec()), Limits::no_limits());
    let _ = <image::codecs::png::PngDecoder<::std::io::Cursor<Vec<u8>>> as image::ImageDecoder>::read_image(&mut png_decoder, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_png_rs_374_9_5cddb6`).

## Output

```
thread 'main' (266261) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/png.rs:374:9:
assertion `left == right` failed
  left: Ok(0)
 right: Ok(1)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<core::result::Result<u64, core::num::error::TryFromIntError>, core::result::Result<u64, core::num::error::TryFromIntError>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <image::codecs::png::PngDecoder<core::io::cursor::Cursor<alloc::vec::Vec<u8>>> as image::io::decoder::ImageDecoder>::read_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/png.rs:374:9
   5: repro_image_src_codecs_png_rs_374_9_5cddb6::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
