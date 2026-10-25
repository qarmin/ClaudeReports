# toml-1.1.8+spec/src/value.rs:389:36

- crate: toml 1.1.8+spec-1.1.0
- kind: panic
- location: `toml-1.1.8+spec/src/value.rs:389:36`
- last API call: `toml::Value::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: Custom("invalid date, expected month between 01 and 12")
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
toml = { version = "=1.1.8+spec-1.1.0" }
toml_datetime = { version = "=1.1.2+spec-1.1.0" }
```

`src/main.rs`:

```rust
use toml::Value;
use toml_datetime::Date;
use toml_datetime::Datetime;

fn main() {
    let value = Value::Datetime(Datetime::from(Date { year: 0u16, month: 0u8, day: 0u8 }));
    let _ = value.to_string();
}
```

Run with `cargo run` in this directory (package `repro_toml_1_1_8_spec_src_value_rs_389_36_c86bb0`).

## Output

```
thread 'main' (376765) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml-1.1.8+spec-1.1.0/src/value.rs:389:36:
called `Result::unwrap()` on an `Err` value: Custom("invalid date, expected month between 01 and 12")
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<&mut alloc::string::String, toml::ser::error::Error>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <toml::value::Value as core::fmt::Display>::fmt
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml-1.1.8+spec-1.1.0/src/value.rs:389:36
   5: <toml::value::Value as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   6: <toml::value::Value as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   7: repro_toml_1_1_8_spec_src_value_rs_389_36_c86bb0::main
             at ./src/main.rs:7:19
   8: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
