# jiff/src/timestamp.rs:2403:37

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/timestamp.rs:2403:37`
- last API call: `jiff::Timestamp::add<jiff::Span>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
adding span to timestamp failed: operation can only be performed with units of hours or smaller, but found non-zero 'day' units (operations on `jiff::Timestamp`, `jiff::tz::Offset` and `jiff::civil::Time` don't support calendar units in a `jiff::Span`)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jiff = { git = "https://github.com/BurntSushi/jiff", rev = "fff640d47f72ca6d3a122678315bbd76488306fe" }
```

`src/main.rs`:

```rust
use jiff::Timestamp;
use jiff::civil::Date;
use jiff::civil::DateTime;
use jiff::civil::Time;

fn main() {
    let timestamp = Timestamp::constant(0i64, 0i32);
    let span = <jiff::civil::DateTime as ::core::ops::Sub<jiff::civil::DateTime>>::sub(DateTime::from(Date::default()), DateTime::from_parts(Date::MAX, Time::MIN));
    let _ = <jiff::Timestamp as ::core::ops::Add<jiff::Span>>::add(timestamp, span);
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_timestamp_rs_2403_37_30ae09`).

## Output

```
thread 'main' (362069) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/timestamp.rs:2403:37:
adding span to timestamp failed: operation can only be performed with units of hours or smaller, but found non-zero 'day' units (operations on `jiff::Timestamp`, `jiff::tz::Offset` and `jiff::civil::Time` don't support calendar units in a `jiff::Span`)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::timestamp::Timestamp, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::timestamp::Timestamp as core::ops::arith::Add<jiff::span::Span>>::add
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/timestamp.rs:2403:37
   5: repro_jiff_src_timestamp_rs_2403_37_30ae09::main
             at ./src/main.rs:9:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
