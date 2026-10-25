# timeout in fraction::prelude::GenericFraction<fraction::BigUint>::is_positive

- crate: fraction 0.17.0
- kind: timeout
- last API call: `fraction::prelude::GenericFraction<fraction::BigUint>::is_positive`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling fraction::prelude::GenericFraction<fraction::BigUint>::is_positive
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fraction = { version = "=0.17.0", features = ["byteorder", "bytes", "juniper", "num-bigint", "postgres-types", "serde", "serde_derive", "with-approx", "with-bigint", "with-decimal", "with-dynaint", "with-juniper-support", "with-postgres-support", "with-serde-support", "with-unicode"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <fraction::prelude::GenericFraction<fraction::BigUint> as fraction::Signed>::is_positive(&<fraction::prelude::GenericFraction<fraction::BigUint> as ::core::convert::From<fraction::approx::sqrt::RawApprox>>::from(<fraction::prelude::GenericFraction<u16>>::nan().sqrt_raw(1688321246u32)));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_390deb`).

