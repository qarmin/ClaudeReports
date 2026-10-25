# timeout in p384::Scalar::mul<p384::Scalar>

- crate: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416
- kind: timeout
- last API call: `p384::Scalar::mul<p384::Scalar>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling p384::Scalar::mul<p384::Scalar>
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
p384 = { git = "https://github.com/RustCrypto/elliptic-curves", rev = "b3a31a0f1b2c265838501bedd842a5e94d31f416" }
```

`src/main.rs`:

```rust
use p384::Scalar;

fn main() {
    let _ = <p384::Scalar as ::core::ops::Mul<p384::Scalar>>::mul(Scalar::from(0u64).sqn_vartime(10000000000000000000usize), Scalar::from(16u32));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_p384_scalar_mul_p384_scalar_a34493`).

