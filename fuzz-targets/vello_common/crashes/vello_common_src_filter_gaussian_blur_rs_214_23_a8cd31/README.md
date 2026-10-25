# vello_common/src/filter/gaussian_blur.rs:214:23

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/filter/gaussian_blur.rs:214:23`
- last API call: `vello_common::filter::gaussian_blur::DecimationSizer::upscale`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15592671846058184006

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
use vello_common::filter::gaussian_blur::DecimationSizer;

fn main() {
    let mut decimation_sizer = DecimationSizer::new(1u16, u16::MAX);
    let _ = decimation_sizer.downscale();
    let _ = decimation_sizer.upscale();
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_filter_gaussian_blur_rs_214_23_a8cd31`).

## Output

```
thread 'main' (706096) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/gaussian_blur.rs:214:23:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <vello_common::filter::gaussian_blur::DecimationSizer>::upscale
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/filter/gaussian_blur.rs:214:23
   4: repro_vello_common_src_filter_gaussian_blur_rs_214_23_a8cd31::main
             at ./src/main.rs:6:30
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
