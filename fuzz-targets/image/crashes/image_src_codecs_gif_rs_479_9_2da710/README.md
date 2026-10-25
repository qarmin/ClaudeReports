# image/src/codecs/gif.rs:479:9

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/codecs/gif.rs:479:9`
- last API call: `image::codecs::gif::GifEncoder<Vec<u8>>::new_with_speed`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 18131996721882149769

## Message

```
speed needs to be in the range [1, 30]
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
    let _ = <image::codecs::gif::GifEncoder<Vec<u8>>>::new_with_speed(vec![], 0i32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_codecs_gif_rs_479_9_2da710`).

## Output

```
thread 'main' (263776) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:479:9:
speed needs to be in the range [1, 30]
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <image::codecs::gif::GifEncoder<alloc::vec::Vec<u8>>>::new_with_speed
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/codecs/gif.rs:479:9
   3: repro_image_src_codecs_gif_rs_479_9_2da710::main
             at ./src/main.rs:2:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
