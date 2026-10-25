# alsa-rs/src/rawmidi.rs:229:40

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: panic
- location: `alsa-rs/src/rawmidi.rs:229:40`
- last API call: `alsa::Rawmidi::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
alsa = { git = "https://github.com/diwic/alsa-rs", rev = "ffc02b01730d2f78d2fdebd26e8ef492bae9f276" }
```

`src/main.rs`:

```rust
use alsa::Direction;
use alsa::Rawmidi;

fn main() {
    let _ = Rawmidi::new("\0", Direction::output(), false);
}
```

Run with `cargo run` in this directory (package `repro_alsa_rs_src_rawmidi_rs_229_40_36cead`).

## Output

```
thread 'main' (1299676) panicked at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/rawmidi.rs:229:40:
called `Result::unwrap()` on an `Err` value: NulError(0, [0])
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<alloc::ffi::c_str::CString, alloc::ffi::c_str::NulError>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <alsa::rawmidi::Rawmidi>::new
             at /home/rafal/.cargo/git/checkouts/alsa-rs-ff4182d1fcf76b3b/ffc02b0/src/rawmidi.rs:229:40
   5: repro_alsa_rs_src_rawmidi_rs_229_40_36cead::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
