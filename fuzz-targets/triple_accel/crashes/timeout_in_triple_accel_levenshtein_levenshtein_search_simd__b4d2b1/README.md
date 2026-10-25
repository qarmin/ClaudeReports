# timeout in triple_accel::levenshtein::levenshtein_search_simd_with_opts

- crate: triple_accel 0.4.0
- kind: timeout
- last API call: `triple_accel::levenshtein::levenshtein_search_simd_with_opts`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling triple_accel::levenshtein::levenshtein_search_simd_with_opts
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
triple_accel = { version = "=0.4.0" }
```

`src/main.rs`:

```rust
use triple_accel::SearchType;
use triple_accel::levenshtein::RDAMERAU_COSTS;

fn main() {
    let _ = triple_accel::levenshtein::levenshtein_search_simd_with_opts(&include_bytes!("data_0.bin").to_vec(), &include_bytes!("data_1.bin").to_vec(), 255u32, SearchType::Best, RDAMERAU_COSTS, true);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_triple_accel_levenshtein_levenshtein_search_simd__b4d2b1`).

