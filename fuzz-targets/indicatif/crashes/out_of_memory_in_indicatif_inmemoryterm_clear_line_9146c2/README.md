# out of memory in indicatif::InMemoryTerm::clear_line

- crate: https://github.com/console-rs/indicatif @ 6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9
- kind: out of memory
- last API call: `indicatif::InMemoryTerm::clear_line`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling indicatif::InMemoryTerm::clear_line
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
indicatif = { git = "https://github.com/console-rs/indicatif", rev = "6132d8d320f62d09d181ed9ae2b0a4dd2ee85cb9", features = ["futures", "improved_unicode", "in_memory", "rayon", "tokio", "unicode-segmentation", "unicode-width", "vt100", "wasmbind"] }
```

`src/main.rs`:

```rust
use indicatif::InMemoryTerm;

fn main() {
    let _ = <indicatif::InMemoryTerm as indicatif::TermLike>::clear_line(&InMemoryTerm::new(10000u16, 10000u16));
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_indicatif_inmemoryterm_clear_line_9146c2`).

## Output

```
memory allocation of 320000 bytes failed
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
