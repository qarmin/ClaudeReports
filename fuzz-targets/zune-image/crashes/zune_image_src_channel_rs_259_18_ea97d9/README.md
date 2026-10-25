# zune-image/src/channel.rs:259:18

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/channel.rs:259:18`
- last API call: `zune_image::frame::Frame::set_channels`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null

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
use zune_core::bit_depth::BitType;
use zune_core::colorspace::ColorSpace;
use zune_image::channel::Channel;
use zune_image::frame::Frame;

fn main() {
    let mut frame = Frame::from_u8(&include_bytes!("data_0.bin").to_vec(), ColorSpace::RGB, 0usize, 0usize);
    frame.set_channels(vec![Channel::new_with_bit_type(4294967296usize, BitType::F32)]);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_channel_rs_259_18_ea97d9`).

## Output

```
thread 'main' (3373435) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:168:30:
unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_nounwind_fmt::runtime
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:122:22
   2: core::panicking::panic_nounwind_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/intrinsics/mod.rs:2636:9
   3: <core::ptr::non_null::NonNull<_>>::new_unchecked::precondition_check
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ub_checks.rs:73:21
   4: <core::ptr::non_null::NonNull<u8>>::new_unchecked
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ub_checks.rs:78:17
   5: alloc::alloc::dealloc
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:168:30
   6: <zune_image::channel::Channel>::dealloc
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:259:18
   7: <zune_image::channel::Channel as core::ops::drop::Drop>::drop
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:657:18
   8: core::ptr::drop_glue::<zune_image::channel::Channel>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
   9: core::ptr::drop_glue::<[zune_image::channel::Channel]>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
  10: core::ptr::drop_in_place::<[zune_image::channel::Channel]>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:842:14
  11: <*mut [zune_image::channel::Channel]>::drop_in_place
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mut_ptr.rs:1384:18
  12: <alloc::vec::Vec<zune_image::channel::Channel> as core::ops::drop::Drop>::drop
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:4358:52
  13: core::ptr::drop_glue::<alloc::vec::Vec<zune_image::channel::Channel>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
  14: core::ptr::drop_glue::<zune_image::frame::Frame>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
  15: repro_zune_image_src_channel_rs_259_18_ea97d9::main
             at ./src/main.rs:9:1
  16: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread caused non-unwinding panic. aborting.
```
