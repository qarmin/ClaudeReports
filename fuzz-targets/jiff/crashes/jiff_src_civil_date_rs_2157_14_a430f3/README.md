# jiff/src/civil/date.rs:2157:14

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/civil/date.rs:2157:14`
- last API call: `jiff::civil::Date::add<jiff::SignedDuration>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
adding signed duration to date overflowed: number of days derived from duration exceed's Jiff's datetime limits: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
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
use jiff::civil::Date;
use jiff::civil::DateTime;

fn main() {
    let date_time = DateTime::from(DateTime::MIN);
    let _ = <jiff::civil::Date as ::core::ops::Add<jiff::SignedDuration>>::add(Date::from(date_time), SignedDuration::new(1000000000000000000i64, 0i32));
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_civil_date_rs_2157_14_a430f3`).

## Output

```
thread 'main' (361686) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/date.rs:2157:14:
adding signed duration to date overflowed: number of days derived from duration exceed's Jiff's datetime limits: parameter 'Unix epoch days' is not in the required range of -4371587..=2932896
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<jiff::civil::date::Date, jiff::error::Error>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <jiff::civil::date::Date as core::ops::arith::Add<jiff::signed_duration::SignedDuration>>::add
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/civil/date.rs:2157:14
   5: repro_jiff_src_civil_date_rs_2157_14_a430f3::main
             at ./src/main.rs:7:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
