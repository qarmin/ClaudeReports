# zune-image/src/frame.rs:322:23

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `zune-image/src/frame.rs:322:23`
- last API call: `zune_image::frame::Frame::insert`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
insertion index (is 1) should be <= len (is 0)
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
use zune_image::channel::Channel;
use zune_image::frame::Frame;

fn main() {
    let bit_type = BitType::F32;
    let mut frame = Frame::new_with_duration(vec![], 0usize, 0usize);
    frame.insert(1usize, Channel::new_with_bit_type(0usize, bit_type));
}
```

Run with `cargo run` in this directory (package `repro_zune_image_src_frame_rs_322_23_fd3fdd`).

## Output

```
thread 'main' (3373275) panicked at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/frame.rs:322:23:
insertion index (is 1) should be <= len (is 0)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <alloc::vec::Vec<_, _>>::insert_mut::assert_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/vec/mod.rs:2339:13
   3: <alloc::vec::Vec<zune_image::channel::Channel>>::insert_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2344:13
   4: <alloc::vec::Vec<zune_image::channel::Channel>>::insert
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2303:22
   5: <zune_image::frame::Frame>::insert
             at /home/rafal/.cargo/git/checkouts/zune-image-827035e253f15ba0/b39e012/crates/zune-image/src/frame.rs:322:23
   6: repro_zune_image_src_frame_rs_322_23_fd3fdd::main
             at ./src/main.rs:8:11
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
