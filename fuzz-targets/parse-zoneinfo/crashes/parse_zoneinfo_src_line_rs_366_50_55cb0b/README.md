# parse-zoneinfo/src/line.rs:366:50

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: panic
- location: `parse-zoneinfo/src/line.rs:366:50`
- last API call: `parse_zoneinfo::line::DaySpec::to_concrete_day`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: "Cannot wrap year from dec->jan"
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
    let day_spec = DaySpec::FirstOnOrAfter(Weekday::Friday, 99i8);
    let _ = day_spec.to_concrete_day(0i64, Month::December);
}
```

Run with `cargo run` in this directory (package `repro_parse_zoneinfo_src_line_rs_366_50_55cb0b`).

## Output

```
thread 'main' (395980) panicked at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:366:50:
called `Result::unwrap()` on an `Err` value: "Cannot wrap year from dec->jan"
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<parse_zoneinfo::line::Month, &str>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:366:50
   5: <&mut <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3} as core::ops::function::FnOnce<(i8,)>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:310:21
   6: <core::option::Option<i8>>::map::<core::option::Option<(parse_zoneinfo::line::Month, i8)>, &mut <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1163:29
   7: <core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:107:26
   8: <core::iter::adapters::fuse::Fuse<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>> as core::iter::adapters::fuse::FuseImpl<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>>>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/fuse.rs:403:29
   9: <core::iter::adapters::fuse::Fuse<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/fuse.rs:51:9
  10: <core::iter::adapters::flatten::FlattenCompat<core::iter::adapters::map::Map<core::ops::range::Range<i8>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}>, core::option::IntoIter<(parse_zoneinfo::line::Month, i8)>> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:855:43
  11: <core::iter::adapters::flatten::FlatMap<core::ops::range::Range<i8>, core::option::Option<(parse_zoneinfo::line::Month, i8)>, <parse_zoneinfo::line::DaySpec>::to_concrete_day::{closure#3}> as core::iter::traits::iterator::Iterator>::next
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/flatten.rs:64:20
  12: <parse_zoneinfo::line::DaySpec>::to_concrete_day
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:375:18
  13: repro_parse_zoneinfo_src_line_rs_366_50_55cb0b::main
             at ./src/main.rs:7:22
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
