# timeout in kurbo::common::solve_itp

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::common::solve_itp`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 563943809284335271

## Message

```
process ended abnormally (timeout) while calling kurbo::common::solve_itp
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = kurbo::common::solve_itp(|_| 1 as f64, 0.0f64, 1.0f64, -0.0f64, 1usize, 0.0f64, f64::MIN, 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_common_solve_itp_3e7515`).

