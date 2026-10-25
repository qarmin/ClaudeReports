# csv-core/src/reader.rs:724:13

- crate: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b
- kind: panic
- location: `csv-core/src/reader.rs:724:13`
- last API call: `csv_core::Reader::read_field`
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
use csv_core::Reader;

fn main() {
    let mut reader = Reader::new();
    reader.set_line(u64::MAX);
    let _ = reader.read_field(&vec![10u8], &mut vec![73u8]);
}
```

Run with `cargo run` in this directory (package `repro_csv_core_src_reader_rs_724_13_a2634e`).

## Output

```
thread 'main' (534268) panicked at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/reader.rs:724:13:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <csv_core::reader::Reader>::read_field_dfa
             at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/reader.rs:724:13
   4: <csv_core::reader::Reader>::read_field
             at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/csv-core/src/reader.rs:544:18
   5: repro_csv_core_src_reader_rs_724_13_a2634e::main
             at ./src/main.rs:6:20
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
