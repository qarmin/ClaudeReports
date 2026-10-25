# out of memory in fast_image_resize::images::ImageRef::new

- crate: fast_image_resize 6.1.0
- kind: out of memory
- last API call: `fast_image_resize::images::ImageRef::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 11636991040402460992

## Message

```
process ended abnormally (out of memory) while calling fast_image_resize::images::ImageRef::new
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
use fast_image_resize::images::ImageRef;

fn main() {
    let vec = vec![];
    let image = Image::new(1u32, 1000000000u32, PixelType::U8x3);
    let _ = ImageRef::new(0u32, 0u32, &vec, image.pixel_type());
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_fast_image_resize_images_imageref_new_531086`).

## Output

```
memory allocation of 3000000000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   8: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: <fast_image_resize::images::image::Image>::new
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/fast_image_resize-6.1.0/src/images/image.rs:120:45
  10: repro_out_of_memory_in_fast_image_resize_images_imageref_new_531086::main
             at ./src/main.rs:7:17
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
