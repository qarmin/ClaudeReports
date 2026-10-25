# image/src/codecs/hdr/encoder.rs:64:44

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/hdr/encoder.rs:64:44`
- last API call: `image::codecs::hdr::HdrEncoder<Vec<u8>>::encode`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7744105691584692531

## Message

```
attempt to multiply with overflow
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
    let hdr_encoder = <image::codecs::hdr::HdrEncoder<Vec<u8>>>::new(vec![]);
    let _ = hdr_encoder.encode(&[], 100usize, 1000000000000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_hdr_encoder_rs_64_44_af8b35`).

## Output

```
thread 'main' (267129) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:64:44:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode_pixels::<core::iter::adapters::map::Map<core::slice::iter::Iter<image::color::Rgb<f32>>, <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode::{closure#0}>>
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:64:44
   4: <image::codecs::hdr::encoder::HdrEncoder<alloc::vec::Vec<u8>>>::encode
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/hdr/encoder.rs:51:14
   5: repro_image_src_codecs_hdr_encoder_rs_64_44_af8b35::main
             at ./src/main.rs:3:25
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
