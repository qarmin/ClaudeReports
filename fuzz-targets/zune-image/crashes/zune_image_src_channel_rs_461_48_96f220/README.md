# zune-image/src/channel.rs:461:48

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/channel.rs:461:48`
- last API call: `zune_image::image::Image::fill[f32]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 7046001469786911260

## Message

```
unsafe precondition(s) violated: ptr::copy requires that both pointer arguments are aligned and non-null

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;

fn main() {
    let _ = Image::fill::<f32>(0.0f32, ColorSpace::BGR, 11usize, 16777217usize);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_channel_rs_461_48_96f220`).

## Output

```
thread 'main' (988364) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:652:9:
unsafe precondition(s) violated: ptr::copy requires that both pointer arguments are aligned and non-null
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_nounwind_fmt::runtime
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:122:22
   2: core::panicking::panic_nounwind_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/intrinsics/mod.rs:2636:9
   3: core::ptr::copy::precondition_check
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ub_checks.rs:73:21
   4: core::ptr::copy::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ub_checks.rs:78:17
   5: <*mut u8>::copy_from
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:1348:18
   6: <zune_image::channel::Channel>::extend_unchecked::<u8>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:461:48
   7: <zune_image::channel::Channel as core::clone::Clone>::clone
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:126:25
   8: <alloc::vec::Vec<zune_image::channel::Channel>>::extend_with
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3725:39
   9: <zune_image::channel::Channel as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:15:11
  10: alloc::vec::from_elem::<zune_image::channel::Channel>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
  11: <zune_image::image::Image>::fill::<f32>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:342:24
  12: repro_zune_image_src_channel_rs_461_48_96f220::main
             at ./src/main.rs:5:13
  13: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread caused non-unwinding panic. aborting.
```
