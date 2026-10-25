# image/src/animation.rs:212:35

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: panic
- location: `image/src/animation.rs:212:35`
- last API call: `image::Delay::from_saturating_duration`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17662178658672334218

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
use image::Delay;

fn main() {
    let _ = Delay::from_saturating_duration(std::time::Duration::new(1000000000u64, 1073741824u32 % 1_000_000_000));
}
```

Run with `cargo run` in this directory (package `repro_image_src_animation_rs_212_35_c92aab`).

## Output

```
thread 'main' (266677) panicked at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/animation.rs:212:35:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <image::animation::Delay>::from_saturating_duration
             at /home/rafal/.cargo/git/checkouts/image-5a5771652f746364/075e65d/src/animation.rs:212:35
   4: repro_image_src_animation_rs_212_35_c92aab::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
