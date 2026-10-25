# out of memory in vello_common::pixmap::Pixmap::try_take_rgb8

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::pixmap::Pixmap::try_take_rgb8`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 15814145987177540561

## Message

```
process ended abnormally (out of memory) while calling vello_common::pixmap::Pixmap::try_take_rgb8
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
peniko = { version = "=0.6.1" }
```

`src/main.rs`:

```rust
use peniko::ImageAlphaType;
use vello_common::pixmap::Pixmap;

fn main() {
    let pixmap = Pixmap::new(u16::MAX, 7357u16);
    let _ = pixmap.clone().try_take_rgb8(ImageAlphaType::AlphaPremultiplied);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_pixmap_pixmap_try_take_rgb8_8b9268`).

## Output

```
memory allocation of 1928563980 bytes failed
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
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<color::rgba8::PremulRgba8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<color::rgba8::PremulRgba8>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <color::rgba8::PremulRgba8 as <[_]>::to_vec_in::ConvertVec>::to_vec::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:448:29
   9: <[color::rgba8::PremulRgba8]>::to_vec_in::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs:400:16
  10: <alloc::vec::Vec<color::rgba8::PremulRgba8> as core::clone::Clone>::clone
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3878:9
  11: <vello_common::pixmap::Pixmap as core::clone::Clone>::clone
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/pixmap.rs:26:5
  12: repro_out_of_memory_in_vello_common_pixmap_pixmap_try_take_rgb8_8b9268::main
             at ./src/main.rs:6:20
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
