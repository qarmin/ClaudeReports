# parse-zoneinfo/src/line.rs:276:18

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: panic
- location: `parse-zoneinfo/src/line.rs:276:18`
- last API call: `parse_zoneinfo::table::RuleInfo::absolute_datetime`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
why is negative modulus designed so?
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
    let rule_info = RuleInfo { from_year: Year::Minimum, to_year: None, month: Month::March, day: DaySpec::FirstOnOrAfter(Weekday::Tuesday, -10i8), time: 0i64, time_type: TimeType::Standard, time_to_add: 0i64, letters: None };
    let _ = rule_info.absolute_datetime(0i64, 0i64, 0i64);
}
```

Run with `cargo run` in this directory (package `repro_parse_zoneinfo_src_line_rs_276_18_e8ab0b`).

## Output

```
thread 'main' (390201) panicked at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:276:18:
why is negative modulus designed so?
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <parse_zoneinfo::line::Weekday>::calculate
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:276:18
   3: <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:360:47
   4: <&mut <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3} as core::ops::function::FnOnce<(i8,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
   5: <core::option::Option<i8>>::map::<core::option::Option<(parse_zoneinfo::line::Month, i8)>, &mut <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
   6: <core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
   7: <core::iter::adapters::fuse::Fuse<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>> as core::iter::adapters::fuse::FuseImpl<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>>>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/fuse.rs:403:29
   8: <core::iter::adapters::fuse::Fuse<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/fuse.rs:51:9
   9: <core::iter::adapters::flatten::FlattenCompat<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>, core::option::IntoIter<(parse_zoneinfo::line::Month, i8)>> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:855:43
  10: <core::iter::adapters::flatten::FlatMap<core::ops::range::Range<i8>, core::option::Option<(parse_zoneinfo::line::Month, i8)>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:64:20
  11: <parse_zoneinfo::line::DaySpec>::to_concrete_day
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:375:18
  12: <parse_zoneinfo::line::ChangeTime>::to_timestamp
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:644:33
  13: <parse_zoneinfo::table::RuleInfo>::absolute_datetime
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/table.rs:144:20
  14: repro_parse_zoneinfo_src_line_rs_276_18_e8ab0b::main
             at ./src/main.rs:10:23
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
