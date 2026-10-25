# timeout in crossbeam_utils::sync::Parker::park

- crate: https://github.com/crossbeam-rs/crossbeam @ 099d0469efcfacb32527bd9451a4fb62586ae537
- kind: timeout
- last API call: `crossbeam_utils::sync::Parker::park`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling crossbeam_utils::sync::Parker::park
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
crossbeam-channel = { git = "https://github.com/crossbeam-rs/crossbeam", rev = "099d0469efcfacb32527bd9451a4fb62586ae537" }
crossbeam-utils = { git = "https://github.com/crossbeam-rs/crossbeam", rev = "099d0469efcfacb32527bd9451a4fb62586ae537" }
```

`src/main.rs`:

```rust
use crossbeam_utils::sync::Parker;

fn main() {
    let parker = Parker::new();
    parker.park();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_crossbeam_utils_sync_parker_park_68e584`).

