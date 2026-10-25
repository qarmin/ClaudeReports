# gimli/src/read/abbrev.rs:495:9

- crate: https://github.com/gimli-rs/gimli @ 64a7a194fb5fe43d474fd9c101555e5a81b1d36c
- kind: panic
- location: `gimli/src/read/abbrev.rs:495:9`
- last API call: `gimli::read::AttributeSpecification::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: (form == constants::DW_FORM_implicit_const && implicit_const_value.is_some())
    ||
    (form != constants::DW_FORM_implicit_const &&
            implicit_const_value.is_none())
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gimli = { git = "https://github.com/gimli-rs/gimli", rev = "64a7a194fb5fe43d474fd9c101555e5a81b1d36c" }
```

`src/main.rs`:

```rust
use gimli::constants::DW_AT_upc_threads_scaled;
use gimli::constants::DW_FORM_ref8;
use gimli::read::AttributeSpecification;

fn main() {
    let _ = AttributeSpecification::new(DW_AT_upc_threads_scaled, DW_FORM_ref8, Some(0i64));
}
```

Run with `cargo run` in this directory (package `repro_gimli_src_read_abbrev_rs_495_9_691710`).

## Output

```
thread 'main' (431020) panicked at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/read/abbrev.rs:495:9:
assertion failed: (form == constants::DW_FORM_implicit_const && implicit_const_value.is_some())
    ||
    (form != constants::DW_FORM_implicit_const &&
            implicit_const_value.is_none())
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <gimli::read::abbrev::AttributeSpecification>::new
             at /home/rafal/.cargo/git/checkouts/gimli-04489b1e1dfc10d2/64a7a19/src/read/abbrev.rs:495:9
   4: repro_gimli_src_read_abbrev_rs_495_9_691710::main
             at ./src/main.rs:6:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
