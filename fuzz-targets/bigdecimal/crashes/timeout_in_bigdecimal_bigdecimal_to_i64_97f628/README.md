# timeout in bigdecimal::BigDecimal::to_i64

- crate: bigdecimal 0.4.11
- kind: timeout
- last API call: `bigdecimal::BigDecimal::to_i64`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling bigdecimal::BigDecimal::to_i64
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
bigdecimal = { version = "=0.4.11" }
```

`src/main.rs`:

```rust
use bigdecimal::BigDecimal;
use bigdecimal::RoundingMode;

fn main() {
    let rounding_mode = RoundingMode::Floor;
    let _ = <bigdecimal::BigDecimal as bigdecimal::ToPrimitive>::to_i64(&BigDecimal::from(1u16).with_precision_round(std::num::NonZero::new(4294967295u64).unwrap(), rounding_mode));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_bigdecimal_bigdecimal_to_i64_97f628`).

