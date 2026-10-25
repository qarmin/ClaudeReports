# out of memory in csv::Writer<Vec<u8>>::write_byte_record

- crate: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b
- kind: out of memory
- last API call: `csv::Writer<Vec<u8>>::write_byte_record`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling csv::Writer<Vec<u8>>::write_byte_record
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
csv = { git = "https://github.com/BurntSushi/rust-csv", rev = "05612e87e6e92910d40d7214b70952b8434fef9b" }
```

`src/main.rs`:

```rust
use csv::ByteRecord;

fn main() {
    let mut writer = <csv::Writer<Vec<u8>>>::from_writer(vec![]);
    let _ = writer.write_byte_record(&ByteRecord::with_capacity(4294967296usize, 0usize));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_csv_writer_vec_u8_write_byte_record_8b9319`).

## Output

```
memory allocation of 4294967296 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:589:25
   6: <alloc::raw_vec::RawVec<u8>>::with_capacity_zeroed_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:221:20
   7: <u8 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_from_elem.rs:51:31
   8: alloc::vec::from_elem::<u8>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3773:5
   9: <csv::byte_record::ByteRecord>::with_capacity
             at /home/rafal/.cargo/git/checkouts/rust-csv-ed0d2e25a059813a/05612e8/src/byte_record.rs:142:21
  10: repro_out_of_memory_in_csv_writer_vec_u8_write_byte_record_8b9319::main
             at ./src/main.rs:5:39
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
