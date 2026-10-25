# zune-image/src/channel.rs:524:34

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/channel.rs:524:34`
- last API call: `zune_image::image::Image::fill[u16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14227473431570616274

## Message

```
unsafe precondition(s) violated: slice::from_raw_parts_mut requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`

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
    let color_space = ColorSpace::RGBA;
    let _ = Image::fill::<u16>(0u16, color_space, 1usize, 4294967295usize);
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_channel_rs_524_34_d271f2`).

## Output

```
thread 'main' (264055) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:524:34:
unsafe precondition(s) violated: slice::from_raw_parts_mut requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_nounwind_fmt::runtime
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:122:22
   2: core::panicking::panic_nounwind_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/intrinsics/mod.rs:2636:9
   3: core::slice::raw::from_raw_parts_mut::precondition_check
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ub_checks.rs:73:21
   4: core::slice::raw::from_raw_parts_mut::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ub_checks.rs:78:17
   5: <zune_image::channel::Channel>::reinterpret_as_mut::<u16>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:524:34
   6: <zune_image::channel::Channel>::fill::<u16>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:600:26
   7: <zune_image::channel::Channel>::from_elm::<u16>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/channel.rs:413:18
   8: <zune_image::image::Image>::fill::<u16>
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/image.rs:342:29
   9: repro_zune_image_src_channel_rs_524_34_d271f2::main
             at ./src/main.rs:6:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread caused non-unwinding panic. aborting.
```
