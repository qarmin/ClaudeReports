# jiff/src/civil/datetime.rs:2728:14

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/civil/datetime.rs:2728:14`
- last API call: `jiff::civil::DateTime::sub<jiff::SignedDuration>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
subtracting signed duration from datetime overflowed: parameter 'days' is not in the required range of -7304484..=7304484
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jiff = { git = "https://github.com/BurntSushi/jiff", rev = "fff640d47f72ca6d3a122678315bbd76488306fe" }
```

`src/main.rs`:

```rust
use jiff::SignedDuration;
use jiff::Timestamp;
use jiff::tz::Offset;

fn main() {
    let _ = <jiff::civil::DateTime as ::core::ops::Sub<jiff::SignedDuration>>::sub(Offset::ZERO.to_datetime(Timestamp::UNIX_EPOCH), SignedDuration::from_micros(-1000000000000000000i64));
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_civil_datetime_rs_2728_14_d19383`).

## Output

```
thread 'main' (361190) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/datetime.rs:2728:14:
subtracting signed duration from datetime overflowed: parameter 'days' is not in the required range of -7304484..=7304484
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::civil::datetime::DateTime, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::civil::datetime::DateTime as core::ops::arith::Sub<jiff::signed_duration::SignedDuration>>::sub
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/datetime.rs:2728:14
   5: repro_jiff_src_civil_datetime_rs_2728_14_d19383::main
             at ./src/main.rs:6:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
