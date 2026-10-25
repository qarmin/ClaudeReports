# timeout in fraction::prelude::GenericFraction<fraction::BigUint>::denom

- crate: https://github.com/dnsl48/fraction @ 53d04cddb4640d1fe7756c289b3ce34be14da504
- kind: timeout
- last API call: `fraction::prelude::GenericFraction<fraction::BigUint>::denom`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling fraction::prelude::GenericFraction<fraction::BigUint>::denom
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
fraction = { git = "https://github.com/dnsl48/fraction", rev = "53d04cddb4640d1fe7756c289b3ce34be14da504", features = ["byteorder", "bytes", "juniper", "num-bigint", "postgres-types", "serde", "serde_derive", "with-approx", "with-bigint", "with-decimal", "with-dynaint", "with-juniper-support", "with-postgres-support", "with-serde-support", "with-unicode"] }
```

`src/main.rs`:

```rust
fn main() {
    let generic_fraction = <fraction::prelude::GenericFraction<fraction::BigUint> as ::core::convert::From<u64>>::from(0u64).sqrt(1000000000u32);
    let _ = generic_fraction.denom();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_fraction_prelude_genericfraction_fraction_biguint_d633b9`).

