# fast_image_resize/src/images/image.rs:120:45

- crate: fast_image_resize 6.1.0
- kind: panic
- location: `fast_image_resize/src/images/image.rs:120:45`
- last API call: `fast_image_resize::images::Image::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 17924529401489662704

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fast_image_resize = { version = "=6.1.0" }
```

`src/main.rs`:

```rust
use fast_image_resize::PixelType;
use fast_image_resize::images::Image;

fn main() {
    let _ = Image::new(1000000000u32, u32::MAX, PixelType::U8x4);
}
```

Run with `cargo run` in this directory (package `repro_fast_image_resize_src_images_image_rs_120_45_8e5e69`).

## Output

```
thread 'main' (1071671) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   5: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   6: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   7: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   8: <fast_image_resize::images::image::Image>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fast_image_resize-6.1.0/src/images/image.rs:120:45
   9: repro_fast_image_resize_src_images_image_rs_120_45_8e5e69::main
             at ./src/main.rs:5:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
