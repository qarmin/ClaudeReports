# v_frame/src/plane/aligned.rs:52:18

- crate: https://github.com/rust-av/v_frame @ d8643c808d96f0cbe30d7f03c0f76a7936a2fe19
- kind: panic
- location: `v_frame/src/plane/aligned.rs:52:18`
- last API call: `v_frame::frame::FrameBuilder::build[u16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
invalid layout
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
    let chroma_subsampling = ChromaSubsampling::Yuv420;
    let frame_builder = FrameBuilder::new(16usize, 1440161638835863816usize, chroma_subsampling, 10u8);
    let frame_builder2 = frame_builder.luma_padding_right(0usize);
    let _ = frame_builder2.build::<u16>();
}
```

Run with `cargo run` in this directory (package `repro_v_frame_src_plane_aligned_rs_52_18_2749d9`).

## Output

```
thread 'main' (434777) panicked at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/aligned.rs:52:18:
invalid layout
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <v_frame::plane::aligned::AlignedData<u16>>::layout
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/aligned.rs:52:18
   3: <v_frame::plane::aligned::AlignedData<u16>>::new
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/aligned.rs:99:22
   4: <v_frame::plane::Plane<u16>>::new
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane.rs:220:19
   5: <v_frame::frame::FrameBuilder>::build::<u16>
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/frame.rs:275:34
   6: repro_v_frame_src_plane_aligned_rs_52_18_2749d9::main
             at ./src/main.rs:8:28
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
