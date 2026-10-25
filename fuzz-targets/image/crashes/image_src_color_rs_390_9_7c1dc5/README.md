# image/src/color.rs:390:9

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/color.rs:390:9`
- last API call: `image::Rgba<u8>::from_slice`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 8162587501057858089

## Message

```
assertion `left == right` failed
  left: 0
 right: 4
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
    let vec = vec![];
    let _ = <image::Rgba<u8> as image::Pixel>::from_slice(&vec);
}
```

Run with `cargo run` in this directory (package `repro_image_src_color_rs_390_9_7c1dc5`).

## Output

```
thread 'main' (264016) panicked at src/main.rs:3:13:
assertion `left == right` failed
  left: 0
 right: 4
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::assert_failed_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:452:17
   3: core::panicking::assert_failed::<usize, usize>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:407:5
   4: <image::color::Rgba<u8> as image::traits::Pixel>::from_slice
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/color.rs:390:9
   5: repro_image_src_color_rs_390_9_7c1dc5::main
             at ./src/main.rs:3:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
