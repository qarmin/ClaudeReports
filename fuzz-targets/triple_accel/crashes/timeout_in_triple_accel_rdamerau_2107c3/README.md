# timeout in triple_accel::rdamerau

- crate: triple_accel 0.4.0
- kind: timeout
- last API call: `triple_accel::rdamerau`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling triple_accel::rdamerau
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
    let vec = triple_accel::alloc_str(65536usize);
    let _ = triple_accel::rdamerau(&vec, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_triple_accel_rdamerau_2107c3`).

