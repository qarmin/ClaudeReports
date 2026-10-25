# parse-zoneinfo/src/line.rs:682:18

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: panic
- location: `parse-zoneinfo/src/line.rs:682:18`
- last API call: `parse_zoneinfo::line::ChangeTime::year`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
parse-zoneinfo = { git = "https://github.com/chronotope/chrono-tz", rev = "64c3e4daa9fa5a4ee4d5a6098609790142552fe5" }
```

`src/main.rs`:

```rust
use parse_zoneinfo::line::ChangeTime;
use parse_zoneinfo::line::DaySpec;
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Weekday;
use parse_zoneinfo::line::Year;

fn main() {
    let day_spec = DaySpec::LastOnOrBefore(Weekday::Sunday, 0i8);
    let year = Year::Maximum;
    let change_time = ChangeTime::UntilDay(year, Month::August, day_spec);
    let _ = change_time.year();
}
```

Run with `cargo run` in this directory (package `repro_parse_zoneinfo_src_line_rs_682_18_638450`).

## Output

```
thread 'main' (390096) panicked at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:682:18:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <parse_zoneinfo::line::ChangeTime>::year
             at /home/rafal/.cargo/git/checkouts/chrono-tz-fac733b9ab166a08/64c3e4d/parse-zoneinfo/src/line.rs:682:18
   4: repro_parse_zoneinfo_src_line_rs_682_18_638450::main
             at ./src/main.rs:11:25
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
