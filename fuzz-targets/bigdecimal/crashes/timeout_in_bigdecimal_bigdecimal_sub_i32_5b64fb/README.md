# timeout in bigdecimal::BigDecimal::sub<i32>

- crate: bigdecimal 0.4.11
- kind: timeout
- last API call: `bigdecimal::BigDecimal::sub<i32>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling bigdecimal::BigDecimal::sub<i32>
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
    let _ = <bigdecimal::BigDecimal as ::core::ops::Sub<i32>>::sub(BigDecimal::from(1i8).with_scale(1000000000i64), 0i32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_bigdecimal_bigdecimal_sub_i32_5b64fb`).

