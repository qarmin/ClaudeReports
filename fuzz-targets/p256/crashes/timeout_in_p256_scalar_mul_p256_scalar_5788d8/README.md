# timeout in p256::Scalar::mul<p256::Scalar>

- crate: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416
- kind: timeout
- last API call: `p256::Scalar::mul<p256::Scalar>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling p256::Scalar::mul<p256::Scalar>
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
p256 = { git = "https://github.com/RustCrypto/elliptic-curves", rev = "b3a31a0f1b2c265838501bedd842a5e94d31f416" }
```

`src/main.rs`:

```rust
use p256::Scalar;

fn main() {
    let _ = <p256::Scalar as ::core::ops::Mul<p256::Scalar>>::mul(Scalar::ZERO, Scalar::default().sqn_vartime(usize::MAX));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_p256_scalar_mul_p256_scalar_5788d8`).

