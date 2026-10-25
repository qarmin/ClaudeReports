# timeout in fraction::prelude::GenericFraction<fraction::BigUint>::sqrt_abs

- crate: fraction 0.17.0
- kind: timeout
- last API call: `fraction::prelude::GenericFraction<fraction::BigUint>::sqrt_abs`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling fraction::prelude::GenericFraction<fraction::BigUint>::sqrt_abs
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
    let generic_fraction = <fraction::prelude::GenericFraction<fraction::BigUint> as ::core::convert::From<i32>>::from(0i32);
    let _ = generic_fraction.sqrt_abs(1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_2cbdf1`).

