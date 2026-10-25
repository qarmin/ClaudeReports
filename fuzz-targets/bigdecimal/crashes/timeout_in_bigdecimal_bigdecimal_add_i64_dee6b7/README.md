# timeout in bigdecimal::BigDecimal::add<i64>

- crate: bigdecimal 0.4.11
- kind: timeout
- last API call: `bigdecimal::BigDecimal::add<i64>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling bigdecimal::BigDecimal::add<i64>
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
    let _ = <bigdecimal::BigDecimal as ::core::ops::Add<i64>>::add(BigDecimal::from(1i32).with_scale_round(-2535105207928642425i64, RoundingMode::Up), 18i64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_bigdecimal_bigdecimal_add_i64_dee6b7`).

