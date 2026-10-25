# timeout in triple_accel::levenshtein::levenshtein_simd_k_with_opts

- crate: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883
- kind: timeout
- last API call: `triple_accel::levenshtein::levenshtein_simd_k_with_opts`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling triple_accel::levenshtein::levenshtein_simd_k_with_opts
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
triple_accel = { git = "https://github.com/Daniel-Liu-c0deb0t/triple_accel", rev = "0f2119a6bbc3e6f007bcca0051660408c47ee883" }
```

`src/main.rs`:

```rust
use triple_accel::levenshtein::EditCosts;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_simd_k_with_opts(&vec![], &include_bytes!("data_0.bin").to_vec(), 2428045950u32, false, EditCosts::new(55u8, 52u8, 134u8, None));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_triple_accel_levenshtein_levenshtein_simd_k_with__850faa`).

