# tracing-subscriber/src/fmt/time/chrono_crate.rs:100:56

- crate: https://github.com/tokio-rs/tracing @ 9d0e557728ecb91e467fc29dfe738b9f28cf6684
- kind: panic
- location: `tracing-subscriber/src/fmt/time/chrono_crate.rs:100:56`
- last API call: `tracing_subscriber::fmt::time::ChronoUtc::format_time`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
a formatting trait implementation returned an error when the underlying stream did not: Error
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tracing-subscriber = { git = "https://github.com/tokio-rs/tracing", rev = "9d0e557728ecb91e467fc29dfe738b9f28cf6684", features = ["alloc", "ansi", "chrono", "env-filter", "fmt", "json", "local-time", "matchers", "nu-ansi-term", "once_cell", "parking_lot", "regex", "registry", "serde", "serde_json", "sharded-slab", "smallvec", "std", "thread_local", "time", "tracing", "tracing-log", "tracing-serde", "valuable", "valuable-serde", "valuable_crate"] }
```

`src/main.rs`:

```rust
use tracing_subscriber::fmt::time::ChronoUtc;

fn main() {
    let mut formatted_fields = <tracing_subscriber::fmt::FormattedFields<tracing_subscriber::fmt::format::Pretty>>::new("".to_string());
    let mut writer = formatted_fields.as_writer();
    let _ = <tracing_subscriber::fmt::time::ChronoUtc as tracing_subscriber::fmt::time::FormatTime>::format_time(&ChronoUtc::new("%".to_string()), &mut writer);
}
```

Run with `cargo run` in this directory (package `repro_tracing_subscriber_src_fmt_time_chrono_crate_rs_100_56_5d1726`).

## Output

```
thread 'main' (1363584) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/fmt.rs:657:14:
a formatting trait implementation returned an error when the underlying stream did not: Error
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(), core::fmt::Error>>::expect
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1184:23
   4: alloc::fmt::format::format_inner
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/fmt.rs:657:14
   5: alloc::fmt::format::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/fmt.rs:661:34
   6: <core::option::Option<&str>>::map_or_else::<alloc::string::String, alloc::fmt::format::{closure#0}, <str as alloc::borrow::ToOwned>::to_owned>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1276:21
   7: alloc::fmt::format
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/fmt.rs:661:19
   8: <tracing_subscriber::fmt::time::chrono_crate::ChronoUtc as tracing_subscriber::fmt::time::FormatTime>::format_time
             at /home/rafal/.cargo/git/checkouts/tracing-cae9f2b327d0db89/9d0e557/tracing-subscriber/src/fmt/time/chrono_crate.rs:100:56
   9: repro_tracing_subscriber_src_fmt_time_chrono_crate_rs_100_56_5d1726::main
             at ./src/main.rs:6:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
