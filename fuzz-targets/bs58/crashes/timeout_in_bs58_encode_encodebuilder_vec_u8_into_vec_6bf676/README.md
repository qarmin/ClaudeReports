# timeout in bs58::encode::EncodeBuilder<Vec<u8>>::into_vec

- crate: https://github.com/Nullus157/bs58-rs @ e4a65a9ec641f26b61fb20518b51e83efd6352a0
- kind: timeout
- last API call: `bs58::encode::EncodeBuilder<Vec<u8>>::into_vec`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling bs58::encode::EncodeBuilder<Vec<u8>>::into_vec
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bs58 = { git = "https://github.com/Nullus157/bs58-rs", rev = "e4a65a9ec641f26b61fb20518b51e83efd6352a0", features = ["alloc", "cb58", "check", "sha2", "smallvec", "std", "tinyvec"] }
```

`src/main.rs`:

```rust
fn main() {
    let encode_builder = bs58::encode::<Vec<u8>>(include_bytes!("data_0.bin").to_vec());
    let encode_builder2 = encode_builder.as_cb58(Some(0u8));
    let _ = encode_builder2.into_vec();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_bs58_encode_encodebuilder_vec_u8_into_vec_6bf676`).

