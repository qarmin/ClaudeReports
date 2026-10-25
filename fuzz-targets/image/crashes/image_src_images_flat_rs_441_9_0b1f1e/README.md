# image/src/images/flat.rs:441:9

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/images/flat.rs:441:9`
- last API call: `image::flat::SampleLayout::in_bounds_index`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
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
    let sample_layout = SampleLayout { channels: 0u8, channel_stride: usize::MAX, width: 0u32, width_stride: 0usize, height: 0u32, height_stride: 1usize };
    let _ = sample_layout.in_bounds_index(1u8, 0u32, 1u32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_images_flat_rs_441_9_0b1f1e`).

## Output

```
thread 'main' (3295669) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/flat.rs:441:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <image::images::flat::SampleLayout>::in_bounds_index
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/images/flat.rs:441:9
   4: repro_image_src_images_flat_rs_441_9_0b1f1e::main
             at ./src/main.rs:5:27
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
