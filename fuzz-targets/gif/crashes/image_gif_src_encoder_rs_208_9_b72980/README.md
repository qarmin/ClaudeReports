# image-gif/src/encoder.rs:208:9

- crate: https://github.com/image-rs/image-gif @ a5f89ddc2b29424334244c97456b4a26e208aa7d
- kind: panic
- location: `image-gif/src/encoder.rs:208:9`
- last API call: `gif::Encoder<Vec<u8>>::write_frame`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8169191176533560612

## Message

```
the frame has 0 pixels, but non-empty buffer
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gif = { git = "https://github.com/image-rs/image-gif", rev = "a5f89ddc2b29424334244c97456b4a26e208aa7d" }
```

`src/main.rs`:

```rust
use gif::Frame;

fn main() {
    let vec = vec![];
    let mut frame = Frame::from_grayscale_with_alpha(0u16, 0u16, &vec);
    frame.make_lzw_pre_encoded();
    let mut encoder = <gif::Encoder<Vec<u8>>>::new(vec![], 0u16, 0u16, &vec![]).unwrap();
    let _ = encoder.write_frame(&frame);
}
```

Run with `cargo run` in this directory (package `repro_image_gif_src_encoder_rs_208_9_b72980`).

## Output

```
thread 'main' (317316) panicked at /home/rafal/.cargo/git/checkouts/image-gif-b1971d000e94ed23/a5f89dd/src/encoder.rs:208:9:
the frame has 0 pixels, but non-empty buffer
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <gif::encoder::Encoder<alloc::vec::Vec<u8>>>::write_frame
             at /home/rafal/.cargo/git/checkouts/image-gif-b1971d000e94ed23/a5f89dd/src/encoder.rs:208:9
   3: repro_image_gif_src_encoder_rs_208_9_b72980::main
             at ./src/main.rs:8:21
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
