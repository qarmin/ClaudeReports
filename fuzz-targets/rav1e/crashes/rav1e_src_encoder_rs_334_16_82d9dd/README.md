# rav1e/src/encoder.rs:334:16

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `rav1e/src/encoder.rs:334:16`
- last API call: `rav1e::prelude::Sequence::get_relative_dist`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rav1e = { git = "https://github.com/xiph/rav1e", rev = "31435de9d76fddd38f6dcc31d4014574cebb2092" }
```

`src/main.rs`:

```rust
use rav1e::EncoderConfig;
use rav1e::prelude::Sequence;

fn main() {
    let sequence = Sequence::new(&EncoderConfig::with_speed_preset(0u8));
    let _ = sequence.get_relative_dist(2498367332u32, 1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_encoder_rs_334_16_82d9dd`).

## Output

```
thread 'main' (313399) panicked at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/encoder.rs:334:16:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rav1e::encoder::Sequence>::get_relative_dist
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/encoder.rs:334:16
   4: repro_rav1e_src_encoder_rs_334_16_82d9dd::main
             at ./src/main.rs:6:22
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
