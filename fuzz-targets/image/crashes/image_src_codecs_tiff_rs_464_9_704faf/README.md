# image/src/codecs/tiff.rs:464:9

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/tiff.rs:464:9`
- last API call: `image::codecs::tiff::TiffDecoder<::std::io::Cursor<Vec<u8>>>::read_image`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 11888221564156234557

## Message

```
assertion `left == right` failed
  left: Ok(0)
 right: Ok(2)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
fn main() {
    let mut tiff_decoder = <image::codecs::tiff::TiffDecoder<::std::io::Cursor<Vec<u8>>>>::new(std::io::Cursor::new(include_bytes!("data_0.bin").to_vec())).unwrap();
    let _ = <image::codecs::tiff::TiffDecoder<::std::io::Cursor<Vec<u8>>> as image::ImageDecoder>::read_image(&mut tiff_decoder, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_tiff_rs_464_9_704faf`).

## Output

```
thread 'main' (265232) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/tiff.rs:464:9:
assertion `left == right` failed
  left: Ok(0)
 right: Ok(2)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<core::result::Result<u64, core::num::error::TryFromIntError>, core::result::Result<u64, core::num::error::TryFromIntError>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:407:5
   4: <image::codecs::tiff::TiffDecoder<core::io::cursor::Cursor<alloc::vec::Vec<u8>>> as image::io::decoder::ImageDecoder>::read_image
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/tiff.rs:464:9
   5: repro_image_src_codecs_tiff_rs_464_9_704faf::main
             at ./src/main.rs:3:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
