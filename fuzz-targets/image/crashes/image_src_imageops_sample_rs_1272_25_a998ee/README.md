# image/src/imageops/sample.rs:1272:25

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/imageops/sample.rs:1272:25`
- last API call: `image::imageops::GaussianBlurParameters::new_anisotropic_kernel_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::imageops::GaussianBlurParameters;

fn main() {
    let _ = GaussianBlurParameters::new_anisotropic_kernel_size(0.357f32, 1.0f32);
}
```

Run with `cargo run` in this directory (package `repro_image_src_imageops_sample_rs_1272_25_a998ee`).

## Output

```
thread 'main' (3295477) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1272:25:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <image::imageops::sample::GaussianBlurParameters>::round_to_nearest_odd
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1272:25
   4: <image::imageops::sample::GaussianBlurParameters>::new_anisotropic_kernel_size
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/imageops/sample.rs:1207:29
   5: repro_image_src_imageops_sample_rs_1272_25_a998ee::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
