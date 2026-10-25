# timeout in triple_accel::levenshtein::levenshtein_simd_k

- crate: https://github.com/Daniel-Liu-c0deb0t/triple_accel @ 0f2119a6bbc3e6f007bcca0051660408c47ee883
- kind: timeout
- last API call: `triple_accel::levenshtein::levenshtein_simd_k`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling triple_accel::levenshtein::levenshtein_simd_k
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
triple_accel = { git = "https://github.com/Daniel-Liu-c0deb0t/triple_accel", rev = "0f2119a6bbc3e6f007bcca0051660408c47ee883" }
```

`src/main.rs`:

```rust
fn main() {
    let vec = triple_accel::alloc_str(65536usize);
    let _ = triple_accel::levenshtein::levenshtein_simd_k(&vec, &vec![], 1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_triple_accel_levenshtein_levenshtein_simd_k_267b39`).

