# timeout in bs58::encode::EncodeBuilder<Vec<u8>>::onto[Vec<u8>]

- crate: https://github.com/Nullus157/bs58-rs @ e4a65a9ec641f26b61fb20518b51e83efd6352a0
- kind: timeout
- last API call: `bs58::encode::EncodeBuilder<Vec<u8>>::onto[Vec<u8>]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling bs58::encode::EncodeBuilder<Vec<u8>>::onto[Vec<u8>]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bs58 = { git = "https://github.com/Nullus157/bs58-rs", rev = "e4a65a9ec641f26b61fb20518b51e83efd6352a0" }
```

`src/main.rs`:

```rust
use bs58::Alphabet;

fn main() {
    let alphabet = Alphabet::FLICKR;
    let encode_builder = <bs58::encode::EncodeBuilder<Vec<u8>>>::new(include_bytes!("data_0.bin").to_vec(), alphabet);
    let _ = encode_builder.onto(include_bytes!("Cargo.toml").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_bs58_encode_encodebuilder_vec_u8_onto_vec_u8_ec3030`).

