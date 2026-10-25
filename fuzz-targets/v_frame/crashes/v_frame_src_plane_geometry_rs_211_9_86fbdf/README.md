# v_frame/src/plane/geometry.rs:211:9

- crate: https://github.com/rust-av/v_frame @ d8643c808d96f0cbe30d7f03c0f76a7936a2fe19
- kind: panic
- location: `v_frame/src/plane/geometry.rs:211:9`
- last API call: `v_frame::plane::PlaneGeometry::data_origin`
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
use v_frame::plane::PlaneGeometry;

fn main() {
    let plane_geometry = PlaneGeometry::new(1usize, 1usize, 0usize, 9499363980073713412usize, 10usize, 0usize, 1u8, 1u8).unwrap();
    let _ = plane_geometry.data_origin();
}
```

Run with `cargo run` in this directory (package `repro_v_frame_src_plane_geometry_rs_211_9_86fbdf`).

## Output

```
thread 'main' (434654) panicked at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/geometry.rs:211:9:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <v_frame::plane::geometry::PlaneGeometry>::data_origin
             at /home/rafal/.cargo/git/checkouts/v_frame-89afcdea4f06af00/d8643c8/src/plane/geometry.rs:211:9
   4: repro_v_frame_src_plane_geometry_rs_211_9_86fbdf::main
             at ./src/main.rs:5:28
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
