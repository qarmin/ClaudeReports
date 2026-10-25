# jiff/src/civil/date.rs:2090:31

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/civil/date.rs:2090:31`
- last API call: `jiff::civil::Date::add<jiff::Span>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
adding span to date overflowed: parameter 'year' is not in the required range of -9999..=9999
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
use jiff::civil::ISOWeekDate;

fn main() {
    let _ = <jiff::civil::Date as ::core::ops::Add<jiff::Span>>::add(ISOWeekDate::MAX.date(), Span::new().try_years::<i64>(1i64).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_civil_date_rs_2090_31_0f01b6`).

## Output

```
thread 'main' (361868) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/date.rs:2090:31:
adding span to date overflowed: parameter 'year' is not in the required range of -9999..=9999
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::civil::date::Date, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::civil::date::Date as core::ops::arith::Add<jiff::span::Span>>::add
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/date.rs:2090:31
   5: repro_jiff_src_civil_date_rs_2090_31_0f01b6::main
             at ./src/main.rs:5:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
