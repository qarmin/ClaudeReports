# vello_common/src/strip.rs:70:9

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/strip.rs:70:9`
- last API call: `vello_common::strip::StripFillSegment::x0`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14202464666003820560

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::strip::StripFillSegment;

fn main() {
    let strip_fill_segment = StripFillSegment { tile_x0: u16::MAX, tile_x1: 0u16, tile_y: 0u16 };
    let _ = strip_fill_segment.x0();
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_strip_rs_70_9_3c92b6`).

## Output

```
thread 'main' (1148668) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/strip.rs:70:9:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <vello_common::strip::StripFillSegment>::x0
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/strip.rs:70:9
   4: repro_vello_common_src_strip_rs_70_9_3c92b6::main
             at ./src/main.rs:5:32
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
