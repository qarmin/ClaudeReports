# timeout in bs58::encode::EncodeBuilder<Vec<u8>>::onto[String]

- crate: https://github.com/Nullus157/bs58-rs @ e4a65a9ec641f26b61fb20518b51e83efd6352a0
- kind: timeout
- last API call: `bs58::encode::EncodeBuilder<Vec<u8>>::onto[String]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling bs58::encode::EncodeBuilder<Vec<u8>>::onto[String]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bs58 = { git = "https://github.com/Nullus157/bs58-rs", rev = "e4a65a9ec641f26b61fb20518b51e83efd6352a0" }
```

`src/main.rs`:

```rust
fn main() {
    let encode_builder = bs58::encode::<Vec<u8>>(include_bytes!("data_0.bin").to_vec());
    let _ = encode_builder.onto("(\\)".to_string());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_bs58_encode_encodebuilder_vec_u8_onto_string_0d206e`).

