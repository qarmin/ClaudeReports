# timeout in epaint::TextureAtlas::new

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: timeout
- last API call: `epaint::TextureAtlas::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 17445521935042210548

## Message

```
process ended abnormally (timeout) while calling epaint::TextureAtlas::new
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
egui = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
epaint = { git = "https://github.com/emilk/egui", rev = "86099aca1276fe786921992c8d5a68914ee79bec" }
```

`src/main.rs`:

```rust
use epaint::FontColorTransferFunction;
use epaint::TextOptions;
use epaint::TextureAtlas;

fn main() {
    let _ = TextureAtlas::new([10000000000000000000usize, 0usize], TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::Off, font_hinting: false, subpixel_binning: false });
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_epaint_textureatlas_new_95d793`).

