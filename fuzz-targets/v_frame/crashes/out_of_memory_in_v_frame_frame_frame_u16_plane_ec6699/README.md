# out of memory in v_frame::frame::Frame<u16>::plane

- crate: https://github.com/rust-av/v_frame @ d8643c808d96f0cbe30d7f03c0f76a7936a2fe19
- kind: out of memory
- last API call: `v_frame::frame::Frame<u16>::plane`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling v_frame::frame::Frame<u16>::plane
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
    let frame = FrameBuilder::new(10usize, 1000000000usize, ChromaSubsampling::Yuv422, 10u8).build::<u16>().unwrap();
    let _ = frame.plane(1usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_v_frame_frame_frame_u16_plane_ec6699`).

## Output

```
memory allocation of 10000000000 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: <v_frame::plane::aligned::AlignedData<u16>>::new
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/aligned.rs:107:25
   5: <v_frame::plane::Plane<u16>>::new
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane.rs:220:19
   6: <v_frame::frame::FrameBuilder>::build::<u16>
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/frame.rs:275:34
   7: repro_out_of_memory_in_v_frame_frame_frame_u16_plane_ec6699::main
             at ./src/main.rs:5:94
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
