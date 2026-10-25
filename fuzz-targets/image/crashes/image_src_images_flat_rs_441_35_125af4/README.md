# image/src/images/flat.rs:441:35

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/images/flat.rs:441:35`
- last API call: `image::flat::SampleLayout::in_bounds_index`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7239672412238398833

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
use image::flat::SampleLayout;

fn main() {
    let sample_layout = SampleLayout { channels: 0u8, channel_stride: 0usize, width: 0u32, width_stride: usize::MAX, height: 1u32, height_stride: 2147483648usize };
    let _ = sample_layout.in_bounds_index(2u8, 4294967294u32, 1212462995u32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_images_flat_rs_441_35_125af4`).

## Output

```
thread 'main' (414577) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/flat.rs:441:35:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <image::images::flat::SampleLayout>::in_bounds_index
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/flat.rs:441:35
   4: repro_image_src_images_flat_rs_441_35_125af4::main
             at ./src/main.rs:5:27
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
