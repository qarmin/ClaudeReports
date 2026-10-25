# timeout in parse_zoneinfo::line::ChangeTime::to_timestamp

- crate: https://github.com/chronotope/chrono-tz @ 64c3e4daa9fa5a4ee4d5a6098609790142552fe5
- kind: timeout
- last API call: `parse_zoneinfo::line::ChangeTime::to_timestamp`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling parse_zoneinfo::line::ChangeTime::to_timestamp
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
use parse_zoneinfo::line::Month;
use parse_zoneinfo::line::Year;

fn main() {
    let change_time = ChangeTime::UntilMonth(Year::Number(1000000000000000000i64), Month::July);
    let _ = change_time.to_timestamp(0i64, 0i64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_parse_zoneinfo_line_changetime_to_timestamp_f7b12c`).

