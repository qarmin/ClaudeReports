# timeout in p256::Scalar::try_from<p256::Scalar>

- crate: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416
- kind: timeout
- last API call: `p256::Scalar::try_from<p256::Scalar>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling p256::Scalar::try_from<p256::Scalar>
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
p256 = { git = "https://github.com/RustCrypto/elliptic-curves", rev = "b3a31a0f1b2c265838501bedd842a5e94d31f416" }
```

`src/main.rs`:

```rust
use p256::NistP256;
use p256::Scalar;

fn main() {
    let _ = NistP256;
    let _ = Scalar::try_from(Scalar::from(0u32).sqn_vartime(usize::MAX));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_p256_scalar_try_from_p256_scalar_486a6a`).

