# timeout in parse_zoneinfo::table::RuleInfo::absolute_datetime

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: timeout
- last API call: `parse_zoneinfo::table::RuleInfo::absolute_datetime`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling parse_zoneinfo::table::RuleInfo::absolute_datetime
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
    let rule_info = RuleInfo { from_year: Year::Maximum, to_year: None, month: Month::October, day: DaySpec::FirstOnOrAfter(Weekday::Tuesday, 0i8), time: 0i64, time_type: TimeType::Standard, time_to_add: 0i64, letters: None };
    let _ = rule_info.absolute_datetime(1000000000000000000i64, 0i64, 0i64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_parse_zoneinfo_table_ruleinfo_absolute_datetime_715975`).

