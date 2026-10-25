# timeout in csv::Writer<Vec<u8>>::write_byte_record

- crate: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b
- kind: timeout
- last API call: `csv::Writer<Vec<u8>>::write_byte_record`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling csv::Writer<Vec<u8>>::write_byte_record
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
use csv::WriterBuilder;

fn main() {
    let mut writer_builder = WriterBuilder::default();
    let _ = writer_builder.buffer_capacity(0usize);
    let mut writer = writer_builder.from_writer::<Vec<u8>>(vec![]);
    let _ = writer.write_byte_record(&ByteRecord::default());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_csv_writer_vec_u8_write_byte_record_099496`).

