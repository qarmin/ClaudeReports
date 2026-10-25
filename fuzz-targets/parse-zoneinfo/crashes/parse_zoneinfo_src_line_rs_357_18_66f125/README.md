# parse-zoneinfo/src/line.rs:357:18

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: panic
- location: `parse-zoneinfo/src/line.rs:357:18`
- last API call: `parse_zoneinfo::table::RuleInfo::absolute_datetime`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
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
use parse_zoneinfo::line::TimeType;
use parse_zoneinfo::line::Weekday;
use parse_zoneinfo::line::Year;
use parse_zoneinfo::table::RuleInfo;

fn main() {
    let day_spec = DaySpec::LastOnOrBefore(Weekday::Saturday, -100i8);
    let rule_info = RuleInfo { from_year: Year::Number(2043728258908512197i64), to_year: Some(Year::Number(62i64)), month: Month::February, day: day_spec, time: -7833470800563864112i64, time_type: TimeType::Wall, time_to_add: i64::MAX, letters: Some("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"56\" height=\"NaN\" viewBox=\"0 0 1e9 142\"><defs><linearGradient id=\"g\"><stop offset=\"0\" stop-color=\"blue\"/><stop offset=\"1\" stop-color=\"yellow\" stop-opacity=\"0.5\"/></linearGradient><clipPath id=\"c\"><circle cx=\"50\" cy=\"50\" r=\"40\"".to_string()) };
    let _ = rule_info.absolute_datetime(i64::MAX, 7i64, 4473909183253428745i64);
}
```

Run with `cargo run` in this directory (package `repro_parse_zoneinfo_src_line_rs_357_18_66f125`).

## Output

```
thread 'main' (391094) panicked at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:357:18:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<(parse_zoneinfo::line::Month, i8)>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <parse_zoneinfo::line::DaySpec>::to_concrete_day
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:357:18
   6: <parse_zoneinfo::line::ChangeTime>::to_timestamp
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:644:33
   7: <parse_zoneinfo::table::RuleInfo>::absolute_datetime
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/table.rs:144:20
   8: repro_parse_zoneinfo_src_line_rs_357_18_66f125::main
             at ./src/main.rs:11:23
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
