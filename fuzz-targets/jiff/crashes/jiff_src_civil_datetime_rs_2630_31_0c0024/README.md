# jiff/src/civil/datetime.rs:2630:31

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/civil/datetime.rs:2630:31`
- last API call: `jiff::civil::DateTime::add<jiff::Span>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
adding span to datetime overflowed: failed to add overflowing span: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jiff = { git = "https://github.com/BurntSushi/jiff", rev = "fff640d47f72ca6d3a122678315bbd76488306fe" }
```

`src/main.rs`:

```rust
use jiff::Span;
use jiff::civil::Date;
use jiff::civil::DateTime;

fn main() {
    let span = Span::try_from(Date::MAX.duration_until(Date::MIN)).unwrap();
    let span2 = span.days::<i64>(0i64);
    let span3 = span2.hours::<i64>(0i64);
    let _ = <jiff::civil::DateTime as ::core::ops::Add<jiff::Span>>::add(DateTime::default(), span3);
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_civil_datetime_rs_2630_31_0c0024`).

## Output

```
thread 'main' (362131) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/datetime.rs:2630:31:
adding span to datetime overflowed: failed to add overflowing span: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::civil::datetime::DateTime, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::civil::datetime::DateTime as core::ops::arith::Add<jiff::span::Span>>::add
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/datetime.rs:2630:31
   5: repro_jiff_src_civil_datetime_rs_2630_31_0c0024::main
             at ./src/main.rs:9:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
