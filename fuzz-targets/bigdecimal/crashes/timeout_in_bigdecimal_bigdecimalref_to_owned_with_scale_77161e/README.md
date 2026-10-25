# timeout in bigdecimal::BigDecimalRef::to_owned_with_scale

- crate: bigdecimal 0.4.11
- kind: timeout
- last API call: `bigdecimal::BigDecimalRef::to_owned_with_scale`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling bigdecimal::BigDecimalRef::to_owned_with_scale
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

fn main() {
    let big_decimal = <bigdecimal::BigDecimal as bigdecimal::Signed>::abs_sub(&BigDecimal::from(0i16), &BigDecimal::from(0i8));
    let big_decimal2 = big_decimal.normalized();
    let big_decimal_ref = big_decimal2.to_ref();
    let _ = big_decimal_ref.to_owned_with_scale(1000000000i64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_bigdecimal_bigdecimalref_to_owned_with_scale_77161e`).

