# contract: CloneEq for zune_image::frame::Frame

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `zune_image::frame::Frame::new_with_duration`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: CloneEq for zune_image::frame::Frame
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false, features = ["bmp", "farbfeld", "hdr", "heic", "jpeg", "jpeg-xl", "jpeg_2000", "png", "ppm", "psd", "qoi", "webp", "metadata", "serde-support", "threads", "simd", "log"] }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::bit_depth::BitType;
use zune_image::channel::Channel;
use zune_image::frame::Frame;

fn main() {
    let frame = Frame::new_with_duration(vec![Channel::new_with_bit_type(10000usize, BitType::F32), Channel::new_with_bit_type(0usize, BitType::U16)], 0usize, 0usize);
    assert!(frame.clone() == frame, "clone is not equal to the original");
}
```

Run with `cargo run` in this directory (package `repro_contract_cloneeq_for_zune_image_frame_frame_65e1b6`).

## Output

```
thread 'main' (3373222) panicked at src/main.rs:7:5:
clone is not equal to the original
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: repro_contract_cloneeq_for_zune_image_frame_frame_65e1b6::main
             at ./src/main.rs:7:5
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
