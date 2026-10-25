# timeout in secp256k1::ecdsa::sign_grind_r

- crate: secp256k1 0.33.1
- kind: timeout
- last API call: `secp256k1::ecdsa::sign_grind_r`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling secp256k1::ecdsa::sign_grind_r
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
secp256k1 = { version = "=0.33.1" }
```

`src/main.rs`:

```rust
use secp256k1::Message;
use secp256k1::SecretKey;

fn main() {
    let _ = secp256k1::ecdsa::sign_grind_r(Message::from_digest([0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 2u8, 0u8, 255u8, 38u8, 2u8, 60u8, 36u8, 255u8, 152u8, 15u8, 232u8, 1u8, 1u8, 222u8, 64u8, 74u8, 255u8, 16u8, 66u8]), &SecretKey::from_secret_bytes([1u8, 15u8, 255u8, 255u8, 232u8, 0u8, 2u8, 205u8, 255u8, 192u8, 114u8, 16u8, 56u8, 255u8, 40u8, 1u8, 0u8, 16u8, 255u8, 16u8, 1u8, 11u8, 1u8, 28u8, 1u8, 99u8, 0u8, 191u8, 255u8, 255u8, 0u8, 45u8]).unwrap(), 9usize);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_secp256k1_ecdsa_sign_grind_r_d32b2e`).

