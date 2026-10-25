# timeout in triple_accel::levenshtein

- crate: triple_accel 0.4.0
- kind: timeout
- last API call: `triple_accel::levenshtein`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling triple_accel::levenshtein
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
triple_accel = { version = "=0.4.0" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = triple_accel::levenshtein(&vec![], &include_bytes!("data_0.bin").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_triple_accel_levenshtein_4c0b3d`).

