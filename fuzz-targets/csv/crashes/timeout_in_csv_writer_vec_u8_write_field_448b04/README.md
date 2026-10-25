# timeout in csv::Writer<Vec<u8>>::write_field

- crate: https://github.com/BurntSushi/rust-csv @ 05612e87e6e92910d40d7214b70952b8434fef9b
- kind: timeout
- last API call: `csv::Writer<Vec<u8>>::write_field`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling csv::Writer<Vec<u8>>::write_field
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
csv = { git = "https://github.com/BurntSushi/rust-csv", rev = "05612e87e6e92910d40d7214b70952b8434fef9b" }
```

`src/main.rs`:

```rust
use csv::WriterBuilder;

fn main() {
    let mut writer_builder = WriterBuilder::new();
    let _ = writer_builder.buffer_capacity(1usize);
    let mut writer = writer_builder.from_writer::<Vec<u8>>(include_bytes!("image_0.jpg").to_vec());
    let _ = writer.write_field::<Vec<u8>>(include_bytes!("rustfmt.toml").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_csv_writer_vec_u8_write_field_448b04`).

