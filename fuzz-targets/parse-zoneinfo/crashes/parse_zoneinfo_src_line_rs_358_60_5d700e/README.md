# parse-zoneinfo/src/line.rs:358:60

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: panic
- location: `parse-zoneinfo/src/line.rs:358:60`
- last API call: `parse_zoneinfo::line::DaySpec::to_concrete_day`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
parse-zoneinfo = { git = "https://github.com/chronotope/chrono-tz", rev = "64c3e4daa9fa5a4ee4d5a6098609790142552fe5" }
```

`src/main.rs`:

```rust
use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Weekday;

fn main() {
    let month = Month::May;
    let day_spec = DaySpec::FirstOnOrAfter(Weekday::Friday, i8::MAX);
    let _ = day_spec.to_concrete_day(0i64, month);
}
```

Run with `cargo run` in this directory (package `repro_parse_zoneinfo_src_line_rs_358_60_5d700e`).

## Output

```
thread 'main' (396130) panicked at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:358:60:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <parse_zoneinfo::line::DaySpec>::to_concrete_day
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:358:60
   4: repro_parse_zoneinfo_src_line_rs_358_60_5d700e::main
             at ./src/main.rs:8:22
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
