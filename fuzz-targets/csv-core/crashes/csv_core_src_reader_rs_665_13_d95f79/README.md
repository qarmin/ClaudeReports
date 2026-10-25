# csv-core/src/reader.rs:665:13

- crate: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b
- kind: panic
- location: `csv-core/src/reader.rs:665:13`
- last API call: `csv_core::Reader::read_record`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
csv-core = { git = "https://github.com/BurntSushi/rust-csv", rev = "05612e87e6e92910d40d7214b70952b8434fef9b" }
```

`src/main.rs`:

```rust
use csv_core::ReaderBuilder;

fn main() {
    let reader_builder = ReaderBuilder::new();
    let mut reader = reader_builder.build();
    reader.set_line(u64::MAX);
    let _ = reader.read_record(&vec![10u8], &mut vec![0u8], &mut vec![0usize]);
}
```

Run with `cargo run` in this directory (package `repro_csv_core_src_reader_rs_665_13_d95f79`).

## Output

```
thread 'main' (534051) panicked at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/reader.rs:665:13:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <csv_core::reader::Reader>::read_record_dfa
             at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/reader.rs:665:13
   4: <csv_core::reader::Reader>::read_record
             at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/reader.rs:601:18
   5: repro_csv_core_src_reader_rs_665_13_d95f79::main
             at ./src/main.rs:7:20
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
