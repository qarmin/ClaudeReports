# timeout in parking::Parker::park

- crate: https://github.com/smol-rs/parking @ cedb1884e0502ad21d4863a21eec9e3f13c3cf5f
- kind: timeout
- last API call: `parking::Parker::park`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling parking::Parker::park
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
parking = { git = "https://github.com/smol-rs/parking", rev = "cedb1884e0502ad21d4863a21eec9e3f13c3cf5f" }
```

`src/main.rs`:

```rust
use parking::Parker;

fn main() {
    let parker = Parker::new();
    parker.park();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_parking_parker_park_1300fb`).

