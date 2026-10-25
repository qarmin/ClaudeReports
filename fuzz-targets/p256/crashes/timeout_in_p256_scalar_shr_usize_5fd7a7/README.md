# timeout in p256::Scalar::shr<usize>

- crate: https://github.com/RustCrypto/elliptic-curves @ b3a31a0f1b2c265838501bedd842a5e94d31f416
- kind: timeout
- last API call: `p256::Scalar::shr<usize>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling p256::Scalar::shr<usize>
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
    let _ = <p256::Scalar as ::core::ops::Shr<usize>>::shr(<p256::Scalar as ::core::ops::Add<p256::Scalar>>::add(Scalar::from(Scalar::from(64u64)).sqn_vartime(usize::MAX), Scalar::from(255u32)), 2147483648usize);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_p256_scalar_shr_usize_5fd7a7`).

