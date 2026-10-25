# timeout in fraction::prelude::GenericFraction<fraction::BigUint>::to_u64

- crate: fraction 0.17.0
- kind: timeout
- last API call: `fraction::prelude::GenericFraction<fraction::BigUint>::to_u64`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling fraction::prelude::GenericFraction<fraction::BigUint>::to_u64
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
    let _ = <fraction::prelude::GenericFraction<fraction::BigUint> as fraction::ToPrimitive>::to_u64(&<fraction::prelude::GenericFraction<fraction::BigInt> as fraction::Zero>::zero().sqrt(1000000000u32));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_3523fe`).

