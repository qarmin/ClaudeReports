# timeout in p256::Scalar::from<p256::Scalar>

- crate: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416
- kind: timeout
- last API call: `p256::Scalar::from<p256::Scalar>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling p256::Scalar::from<p256::Scalar>
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
    let scalar = <p256::Scalar as ::core::ops::Add<p256::Scalar>>::add(Scalar::from(0u64), <p256::Scalar as ::core::ops::Neg>::neg(Scalar::from(u32::MAX)).sqn_vartime(2usize));
    let scalar2 = scalar.shr_vartime(3880915770u32);
    let scalar3 = <p256::Scalar as ::core::ops::Neg>::neg(scalar2);
    let _ = Scalar::from(scalar3.sqn_vartime(2147483648usize));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_p256_scalar_from_p256_scalar_7f5503`).

