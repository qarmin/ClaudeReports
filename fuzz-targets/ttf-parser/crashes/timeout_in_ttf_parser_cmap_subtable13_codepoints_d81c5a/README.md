# timeout in ttf_parser::cmap::Subtable13::codepoints

- crate: https://github.com/harfbuzz/ttf-parser @ 0c7291223fe9e0bc2808f0255cb991066eedd796
- kind: timeout
- last API call: `ttf_parser::cmap::Subtable13::codepoints`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 18211742397380861367

## Message

```
process ended abnormally (timeout) while calling ttf_parser::cmap::Subtable13::codepoints
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ttf-parser = { git = "https://github.com/harfbuzz/ttf-parser", rev = "0c7291223fe9e0bc2808f0255cb991066eedd796" }
```

`src/main.rs`:

```rust
use ttf_parser::cmap::Subtable13;

fn main() {
    let vec = include_bytes!("data_0.bin").to_vec();
    let subtable13 = Subtable13::parse(&vec).unwrap();
    subtable13.codepoints(|_| ());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_ttf_parser_cmap_subtable13_codepoints_d81c5a`).

