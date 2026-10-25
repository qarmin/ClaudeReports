# rav1e/src/transform/mod.rs:328:14

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `rav1e/src/transform/mod.rs:328:14`
- last API call: `rav1e::bench::transform::av1_round_shift_array`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to negate with overflow
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
    rav1e::bench::transform::av1_round_shift_array(&mut vec![0i32], 1usize, i8::MIN);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_transform_mod_rs_328_14_82f515`).

## Output

```
thread 'main' (310209) panicked at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/mod.rs:328:14:
attempt to negate with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_neg_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: rav1e::transform::av1_round_shift_array::{closure#1}
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/mod.rs:328:14
   4: <core::iter::adapters::take::Take<core::slice::iter::IterMut<i32>> as core::iter::adapters::take::SpecTake>::spec_for_each::<rav1e::transform::av1_round_shift_array::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/take.rs:329:13
   5: <core::iter::adapters::take::Take<core::slice::iter::IterMut<i32>> as core::iter::traits::iterator::Iterator>::for_each::<rav1e::transform::av1_round_shift_array::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/take.rs:126:9
   6: rav1e::transform::av1_round_shift_array
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/mod.rs:327:31
   7: repro_rav1e_src_transform_mod_rs_328_14_82f515::main
             at ./src/main.rs:2:5
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
