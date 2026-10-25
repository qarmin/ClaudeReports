# timeout in p256::Scalar::sub<p256::Scalar>

- crate: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416
- kind: timeout
- last API call: `p256::Scalar::sub<p256::Scalar>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling p256::Scalar::sub<p256::Scalar>
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
    let scalar = Scalar::from(0u64);
    let scalar2 = scalar.double();
    let _ = <p256::Scalar as ::core::ops::Sub<p256::Scalar>>::sub(scalar2.sqn_vartime(usize::MAX), Scalar::from(32327102942228415u64));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_p256_scalar_sub_p256_scalar_ea086d`).

