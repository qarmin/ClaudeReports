# timeout in alsa::poll::poll

- crate: https://github.com/diwic/alsa-rs @ ffc02b01730d2f78d2fdebd26e8ef492bae9f276
- kind: timeout
- last API call: `alsa::poll::poll`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling alsa::poll::poll
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
alsa = { git = "https://github.com/diwic/alsa-rs", rev = "ffc02b01730d2f78d2fdebd26e8ef492bae9f276" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = alsa::poll::poll(&mut vec![], -1000000000i32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_alsa_poll_poll_03cbeb`).

