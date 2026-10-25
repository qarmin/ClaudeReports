# rav1e/src/ec.rs:662:20

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `rav1e/src/ec.rs:662:20`
- last API call: `rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder>::write_unsigned_subexp_with_ref`
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
rav1e = { git = "https://github.com/xiph/rav1e", rev = "31435de9d76fddd38f6dcc31d4014574cebb2092", features = ["aom-sys", "asm", "av-metrics", "backtrace", "bench", "binaries", "byteorder", "capi", "cc", "channel-api", "check_asm", "clap", "clap_complete", "console", "crossbeam", "dav1d-sys", "decode_test", "decode_test_dav1d", "dump_ivf", "dump_lookahead_data", "fern", "git_version", "image", "ivf", "nasm-rs", "nom", "quick_test", "scan_fmt", "serde", "serde-big-array", "serialize", "signal-hook", "signal_support", "threading", "toml", "tracing", "tracing-chrome", "tracing-subscriber", "unstable", "wasm", "wasm-bindgen", "y4m"] }
```

`src/main.rs`:

```rust
use rav1e::bench::ec::WriterEncoder;

fn main() {
    let mut writer_base = WriterEncoder::new();
    <rav1e::bench::ec::WriterBase<rav1e::bench::ec::WriterEncoder> as rav1e::bench::ec::Writer>::write_unsigned_subexp_with_ref(&mut writer_base, 0u32, 0u32, 31u8, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_ec_rs_662_20_6aa3ad`).

## Output

```
thread 'main' (313633) panicked at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/ec.rs:662:20:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rav1e::ec::WriterBase<rav1e::ec::WriterEncoder> as rav1e::ec::Writer>::write_subexp
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/ec.rs:662:20
   4: <rav1e::ec::WriterBase<rav1e::ec::WriterEncoder> as rav1e::ec::Writer>::write_unsigned_subexp_with_ref
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/ec.rs:717:12
   5: repro_rav1e_src_ec_rs_662_20_6aa3ad::main
             at ./src/main.rs:5:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
