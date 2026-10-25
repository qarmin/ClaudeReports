# timeout in textwrap::wrap_columns

- crate: https://github.com/mgeisler/textwrap @ 07fdbbb92103a22c48a61be271dca36c739fdde3
- kind: timeout
- last API call: `textwrap::wrap_columns`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling textwrap::wrap_columns
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
textwrap = { git = "https://github.com/mgeisler/textwrap", rev = "07fdbbb92103a22c48a61be271dca36c739fdde3", features = ["hyphenation", "icu_segmenter", "smawk", "std", "terminal_size", "unicode-linebreak", "unicode-linebreak-complex", "unicode-width"] }
```

`src/main.rs`:

```rust
use textwrap::Options;
use textwrap::WrapAlgorithm;

fn main() {
    let wrap_algorithm = WrapAlgorithm::default();
    let options = Options::with_termwidth();
    let options2 = options.clone().wrap_algorithm(wrap_algorithm);
    let _ = textwrap::wrap_columns::<textwrap::Options>("", 4294967296usize, options2.clone(), "\u{feff}", "", "\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}\u{200b}\u{200d}");
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_textwrap_wrap_columns_2eb088`).

