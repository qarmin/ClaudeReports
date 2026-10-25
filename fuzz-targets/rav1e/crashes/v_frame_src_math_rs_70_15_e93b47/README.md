# v_frame/src/math.rs:70:15

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `v_frame/src/math.rs:70:15`
- source: https://docs.rs/crate/v_frame/0.3.9/source/src/math.rs#70
- last API call: `rav1e::bench::transform::av1_round_shift_array`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to shift left with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rav1e = { git = "https://github.com/xiph/rav1e", rev = "31435de9d76fddd38f6dcc31d4014574cebb2092", features = ["aom-sys", "asm", "av-metrics", "backtrace", "bench", "binaries", "byteorder", "capi", "cc", "channel-api", "check_asm", "clap", "clap_complete", "console", "crossbeam", "dav1d-sys", "decode_test", "decode_test_dav1d", "dump_ivf", "dump_lookahead_data", "fern", "git_version", "image", "ivf", "nasm-rs", "nom", "quick_test", "scan_fmt", "serde", "serde-big-array", "serialize", "signal-hook", "signal_support", "threading", "toml", "tracing", "tracing-chrome", "tracing-subscriber", "unstable", "wasm", "wasm-bindgen", "y4m"] }
```

`src/main.rs`:

```rust
fn main() {
    rav1e::bench::transform::av1_round_shift_array(&mut vec![0i32], 1usize, 37i8);
}
```

Run with `cargo run` in this directory (package `repro_v_frame_src_math_rs_70_15_e93b47`).

## Output

```
thread 'main' (310709) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/v_frame-0.3.9/src/math.rs:70:15:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: v_frame::math::round_shift
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/v_frame-0.3.9/src/math.rs:70:15
   4: rav1e::transform::av1_round_shift_array::{closure#0}
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/mod.rs:324:12
   5: <core::iter::adapters::take::Take<core::slice::iter::IterMut<i32>> as core::iter::adapters::take::SpecTake>::spec_for_each::<rav1e::transform::av1_round_shift_array::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/take.rs:329:13
   6: <core::iter::adapters::take::Take<core::slice::iter::IterMut<i32>> as core::iter::traits::iterator::Iterator>::for_each::<rav1e::transform::av1_round_shift_array::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/take.rs:126:9
   7: rav1e::transform::av1_round_shift_array
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/mod.rs:323:31
   8: repro_v_frame_src_math_rs_70_15_e93b47::main
             at ./src/main.rs:2:5
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
