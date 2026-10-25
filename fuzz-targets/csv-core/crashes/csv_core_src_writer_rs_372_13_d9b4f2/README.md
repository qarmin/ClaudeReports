# csv-core/src/writer.rs:372:13

- crate: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b
- kind: panic
- location: `csv-core/src/writer.rs:372:13`
- last API call: `csv_core::Writer::terminator`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: !self.state.quoting
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
csv-core = { git = "https://github.com/BurntSushi/rust-csv", rev = "05612e87e6e92910d40d7214b70952b8434fef9b" }
```

`src/main.rs`:

```rust
use csv_core::WriterBuilder;

fn main() {
    let writer_builder = WriterBuilder::default();
    let mut writer = writer_builder.build();
    let _ = writer.field(&vec![10u8], &mut vec![]);
    let _ = writer.terminator(&mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_csv_core_src_writer_rs_372_13_d9b4f2`).

## Output

```
thread 'main' (534148) panicked at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/writer.rs:372:13:
assertion failed: !self.state.quoting
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <csv_core::writer::Writer>::terminator
             at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/writer.rs:372:13
   4: repro_csv_core_src_writer_rs_372_13_d9b4f2::main
             at ./src/main.rs:7:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
