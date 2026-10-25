# rav1e/src/ec.rs:627:15

- crate: rav1e 0.8.1
- kind: panic
- location: `rav1e/src/ec.rs:627:15`
- source: https://docs.rs/crate/rav1e/0.8.1/source/src/ec.rs#627
- last API call: `rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder>::write_quniform`
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
rav1e = { version = "=0.8.1", features = ["aom-sys", "asm", "av-metrics", "backtrace", "bench", "binaries", "byteorder", "capi", "cc", "channel-api", "check_asm", "clap", "clap_complete", "console", "crossbeam", "dav1d-sys", "decode_test", "decode_test_dav1d", "dump_ivf", "dump_lookahead_data", "fern", "git_version", "image", "ivf", "nasm-rs", "nom", "quick_test", "scan_fmt", "serde", "serde-big-array", "serialize", "signal-hook", "signal_support", "threading", "toml", "tracing", "tracing-chrome", "tracing-subscriber", "unstable", "wasm", "wasm-bindgen", "y4m"] }
```

`src/main.rs`:

```rust
use rav1e::bench::ec::WriterRecorder;

fn main() {
    let mut writer_base = WriterRecorder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterRecorder> as rav1e::bench::ec::Writer>::write_quniform(&mut writer_base, u32::MAX, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_ec_rs_627_15_6b4082`).

## Output

```
thread 'main' (2413163) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rav1e-0.8.1/src/ec.rs:627:15:
attempt to shift left with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_shl_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rav1e::ec::WriterBase<rav1e::ec::WriterRecorder> as rav1e::ec::Writer>::write_quniform
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rav1e-0.8.1/src/ec.rs:627:15
   4: repro_rav1e_src_ec_rs_627_15_6b4082::main
             at ./src/main.rs:5:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
