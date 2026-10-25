# alsa-rs/src/mixer.rs:123:48

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: panic
- location: `alsa-rs/src/mixer.rs:123:48`
- last API call: `alsa::mixer::MilliBel::sub<alsa::mixer::MilliBel>`
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
alsa = { git = "https://github.com/diwic/alsa-rs", rev = "ffc02b01730d2f78d2fdebd26e8ef492bae9f276" }
```

`src/main.rs`:

```rust
use alsa::mixer::MilliBel;

fn main() {
    let _ = <alsa::mixer::MilliBel as ::core::ops::Sub<alsa::mixer::MilliBel>>::sub(MilliBel(i64::MIN), MilliBel::from_db(1.0f32));
}
```

Run with `cargo run` in this directory (package `repro_alsa_rs_src_mixer_rs_123_48_5fda4b`).

## Output

```
thread 'main' (1299720) panicked at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/mixer.rs:123:48:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <alsa::mixer::MilliBel as core::ops::arith::Sub>::sub
             at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/mixer.rs:123:48
   4: repro_alsa_rs_src_mixer_rs_123_48_5fda4b::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
