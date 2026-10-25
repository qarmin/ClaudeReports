# timeout in ark_ff::fields::utils::k_adicity

- crate: https://github.com/arkworks-rs/algebra @ 4a19a1c50606ae4a0e96ccce107f6281aa0be38e
- kind: timeout
- last API call: `ark_ff::fields::utils::k_adicity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling ark_ff::fields::utils::k_adicity
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ark-ff = { git = "https://github.com/arkworks-rs/algebra", rev = "4a19a1c50606ae4a0e96ccce107f6281aa0be38e" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = ark_ff::fields::utils::k_adicity(1u64, 1u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_ark_ff_fields_utils_k_adicity_04d269`).

