# jiff/src/fmt/temporal/printer.rs:160:9

- crate: https://github.com/BurntSushi/jiff @ fff640d47f72ca6d3a122678315bbd76488306fe
- kind: panic
- location: `jiff/src/fmt/temporal/printer.rs:160:9`
- last API call: `jiff::fmt::temporal::DateTimePrinter::separator`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
RFC3339 separator must be ASCII
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jiff = { git = "https://github.com/BurntSushi/jiff", rev = "fff640d47f72ca6d3a122678315bbd76488306fe" }
```

`src/main.rs`:

```rust
use jiff::fmt::temporal::DateTimePrinter;

fn main() {
    let date_time_printer = DateTimePrinter::new();
    let _ = date_time_printer.separator(255u8);
}
```

Run with `cargo run` in this directory (package `repro_jiff_src_fmt_temporal_printer_rs_160_9_fae1d0`).

## Output

```
thread 'main' (1272613) panicked at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/fmt/temporal/printer.rs:160:9:
RFC3339 separator must be ASCII
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <jiff::fmt::temporal::printer::DateTimePrinter>::separator
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/fmt/temporal/printer.rs:160:9
   3: <jiff::fmt::temporal::DateTimePrinter>::separator
             at /home/rafal/.cargo/git/checkouts/jiff-1e5079547c1aaa1b/fff640d/crates/jiff/src/fmt/temporal/mod.rs:1262:25
   4: repro_jiff_src_fmt_temporal_printer_rs_160_9_fae1d0::main
             at ./src/main.rs:5:31
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
