# rav1e/src/ec.rs:534:5

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `rav1e/src/ec.rs:534:5`
- last API call: `rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter>::symbol[4]`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 2144337269094167553

## Message

```
assertion failed: (fh >> EC_PROB_SHIFT) <= (fl >> EC_PROB_SHIFT)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rav1e = { git = "https://github.com/xiph/rav1e", rev = "31435de9d76fddd38f6dcc31d4014574cebb2092", features = ["aom-sys", "asm", "av-metrics", "backtrace", "bench", "binaries", "byteorder", "capi", "cc", "channel-api", "check_asm", "clap", "clap_complete", "console", "crossbeam", "dav1d-sys", "decode_test", "decode_test_dav1d", "dump_ivf", "dump_lookahead_data", "fern", "git_version", "image", "ivf", "nasm-rs", "nom", "quick_test", "scan_fmt", "serde", "serde-big-array", "serialize", "signal-hook", "signal_support", "threading", "toml", "tracing", "tracing-chrome", "tracing-subscriber", "unstable", "wasm", "wasm-bindgen", "y4m"] }
```

`src/main.rs`:

```rust
use rav1e::bench::ec::WriterCounter;

fn main() {
    let mut writer_base = WriterCounter::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterCounter> as rav1e::bench::ec::Writer>::symbol::<4>(&mut writer_base, 0u32, &[u16::MAX, 0u16, 0u16, 1u16]);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_ec_rs_534_5_f8415a`).

## Output

```
thread 'main' (563841) panicked at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/ec.rs:534:5:
assertion failed: (fh >> EC_PROB_SHIFT) <= (fl >> EC_PROB_SHIFT)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <rav1e::ec::WriterBase<rav1e::ec::WriterCounter> as rav1e::ec::Writer>::symbol::<4>
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/ec.rs:534:5
   4: repro_rav1e_src_ec_rs_534_5_f8415a::main
             at ./src/main.rs:5:5
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
