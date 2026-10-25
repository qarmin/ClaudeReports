# v_frame/src/plane/aligned.rs:48:14

- crate: https://github.com/rust-av/v_frame @ d8643c808d96f0cbe30d7f03c0f76a7936a2fe19
- kind: panic
- location: `v_frame/src/plane/aligned.rs:48:14`
- last API call: `v_frame::frame::FrameBuilder::build[u16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
allocation size does not overflow usize
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
v_frame = { git = "https://github.com/rust-av/v_frame", rev = "d8643c808d96f0cbe30d7f03c0f76a7936a2fe19" }
```

`src/main.rs`:

```rust
use v_frame::chroma::ChromaSubsampling;
use v_frame::frame::FrameBuilder;

fn main() {
    let frame_builder = FrameBuilder::new(4294967295usize, 4294967295usize, ChromaSubsampling::Monochrome, 10u8);
    let _ = frame_builder.build::<u16>();
}
```

Run with `cargo run` in this directory (package `repro_v_frame_src_plane_aligned_rs_48_14_e2d10f`).

## Output

```
thread 'main' (434375) panicked at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/aligned.rs:48:14:
allocation size does not overflow usize
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<usize>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <v_frame::plane::aligned::AlignedData<u16>>::layout
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/aligned.rs:48:14
   6: <v_frame::plane::aligned::AlignedData<u16>>::new
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/aligned.rs:99:22
   7: <v_frame::plane::Plane<u16>>::new
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane.rs:220:19
   8: <v_frame::frame::FrameBuilder>::build::<u16>
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/frame.rs:281:22
   9: repro_v_frame_src_plane_aligned_rs_48_14_e2d10f::main
             at ./src/main.rs:6:27
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
