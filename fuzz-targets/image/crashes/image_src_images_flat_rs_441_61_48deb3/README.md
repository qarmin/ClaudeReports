# image/src/images/flat.rs:441:61

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/images/flat.rs:441:61`
- last API call: `image::flat::SampleLayout::in_bounds_index`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 4273540737589619343

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
    let sample_layout = SampleLayout { channels: 0u8, channel_stride: usize::MAX, width: 3632196899u32, width_stride: 1usize, height: 13u32, height_stride: 2usize };
    let _ = sample_layout.in_bounds_index(232u8, 16u32, 8u32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_images_flat_rs_441_61_48deb3`).

## Output

```
thread 'main' (525501) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/flat.rs:441:61:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <image::images::flat::SampleLayout>::in_bounds_index
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/flat.rs:441:61
   4: repro_image_src_images_flat_rs_441_61_48deb3::main
             at ./src/main.rs:5:27
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
