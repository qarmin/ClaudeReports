# timeout in fraction::prelude::GenericFraction<u16>::sqrt

- crate: fraction 0.17.0
- kind: timeout
- last API call: `fraction::prelude::GenericFraction<u16>::sqrt`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling fraction::prelude::GenericFraction<u16>::sqrt
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
    let generic_fraction = <fraction::prelude::GenericFraction<u16> as fraction::Bounded>::min_value();
    let generic_fraction2 = generic_fraction.fract();
    let _ = generic_fraction2.sqrt(1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_fraction_prelude_genericfraction_u16_sqrt_f01a4d`).

