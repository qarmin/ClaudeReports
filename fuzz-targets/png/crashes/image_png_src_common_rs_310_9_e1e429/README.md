# image-png/src/common.rs:310:9

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/common.rs:310:9`
- last API call: `png::FrameControl::inc_seq_num`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
png = { git = "https://github.com/image-rs/image-png", rev = "b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc" }
```

`src/main.rs`:

```rust
use png::BlendOp;
use png::DisposeOp;
use png::FrameControl;

fn main() {
    let mut frame_control = FrameControl { sequence_number: u32::MAX, width: 0u32, height: 0u32, x_offset: 0u32, y_offset: 0u32, delay_num: 0u16, delay_den: 0u16, dispose_op: DisposeOp::Previous, blend_op: BlendOp::Over };
    frame_control.inc_seq_num(1u32);
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_common_rs_310_9_e1e429`).

## Output

```
thread 'main' (1263932) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/common.rs:310:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <png::common::FrameControl>::inc_seq_num
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/common.rs:310:9
   4: repro_image_png_src_common_rs_310_9_e1e429::main
             at ./src/main.rs:7:19
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
