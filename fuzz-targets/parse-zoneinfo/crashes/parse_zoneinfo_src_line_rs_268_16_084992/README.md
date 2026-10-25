# parse-zoneinfo/src/line.rs:268:16

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: panic
- location: `parse-zoneinfo/src/line.rs:268:16`
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
    let day_spec = DaySpec::Last(Weekday::Saturday);
    let _ = day_spec.to_concrete_day(i64::MIN, Month::July);
}
```

Run with `cargo run` in this directory (package `repro_parse_zoneinfo_src_line_rs_268_16_084992`).

## Output

```
thread 'main' (390376) panicked at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:268:16:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <parse_zoneinfo::line::Weekday>::calculate
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:268:16
   4: <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#1}
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:335:34
   5: core::iter::traits::double_ended::DoubleEndedIterator::rfind::check::<i8, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#1}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/double_ended.rs:427:20
   6: <core::ops::range::Range<i8> as core::iter::traits::double_ended::DoubleEndedIterator>::try_rfold::<(), core::iter::traits::double_ended::DoubleEndedIterator::rfind::check<i8, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#1}>::{closure#0}, core::ops::control_flow::ControlFlow<i8>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/double_ended.rs:290:21
   7: <core::ops::range::Range<i8> as core::iter::traits::double_ended::DoubleEndedIterator>::rfind::<<parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/double_ended.rs:431:14
   8: <core::iter::adapters::rev::Rev<core::ops::range::Range<i8>> as core::iter::traits::iterator::Iterator>::find::<<parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#1}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/rev.rs:91:19
   9: <parse_zoneinfo::line::DaySpec>::to_concrete_day
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:335:22
  10: repro_parse_zoneinfo_src_line_rs_268_16_084992::main
             at ./src/main.rs:7:22
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
