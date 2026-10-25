# toml_datetime-1.1.2+spec/src/datetime.rs:285:21

- crate: toml 1.1.8+spec-1.1.0
- kind: panic
- location: `toml_datetime-1.1.2+spec/src/datetime.rs:285:21`
- last API call: `toml_datetime::Offset::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
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
use toml_datetime::Offset;

fn main() {
    let offset = Offset::Custom { minutes: i16::MIN };
    let _ = offset.to_string();
}
```

Run with `cargo run` in this directory (package `repro_toml_datetime_1_1_2_spec_src_datetime_rs_285_21_f8a03c`).

## Output

```
thread 'main' (376170) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml_datetime-1.1.2+spec-1.1.0/src/datetime.rs:285:21:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <toml_datetime::datetime::Offset as core::fmt::Display>::fmt
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/toml_datetime-1.1.2+spec-1.1.0/src/datetime.rs:285:21
   4: <toml_datetime::datetime::Offset as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   5: <toml_datetime::datetime::Offset as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   6: repro_toml_datetime_1_1_2_spec_src_datetime_rs_285_21_f8a03c::main
             at ./src/main.rs:5:20
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
