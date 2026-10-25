# timeout in p384::Scalar::sqn_vartime

- crate: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416
- kind: timeout
- last API call: `p384::Scalar::sqn_vartime`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling p384::Scalar::sqn_vartime
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
    let scalar = Scalar::try_from(Scalar::ZERO.sqn_vartime(65536usize)).unwrap();
    let scalar2 = scalar.add(&Scalar::default());
    let _ = scalar2.sqn_vartime(4294967296usize);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_p384_scalar_sqn_vartime_cfd136`).

