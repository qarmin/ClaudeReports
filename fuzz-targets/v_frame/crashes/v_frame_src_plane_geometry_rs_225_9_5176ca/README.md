# v_frame/src/plane/geometry.rs:225:9

- crate: https://github.com/rust-av/v_frame @ d8643c808d96f0cbe30d7f03c0f76a7936a2fe19
- kind: panic
- location: `v_frame/src/plane/geometry.rs:225:9`
- last API call: `v_frame::frame::FrameBuilder::build[u16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
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
    let frame_builder = FrameBuilder::new(100usize, 10000000000000000000usize, ChromaSubsampling::Yuv444, 16u8);
    let _ = frame_builder.build::<u16>();
}
```

Run with `cargo run` in this directory (package `repro_v_frame_src_plane_geometry_rs_225_9_5176ca`).

## Output

```
thread 'main' (434470) panicked at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/geometry.rs:225:9:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <v_frame::plane::geometry::PlaneGeometry>::alloc_size
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/geometry.rs:225:9
   4: <v_frame::plane::Plane<u16>>::new
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane.rs:217:31
   5: <v_frame::frame::FrameBuilder>::build::<u16>
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/frame.rs:275:34
   6: repro_v_frame_src_plane_geometry_rs_225_9_5176ca::main
             at ./src/main.rs:6:27
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
