# timeout in epaint::TextureAtlas::options

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: timeout
- last API call: `epaint::TextureAtlas::options`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 3444106002893291787

## Message

```
process ended abnormally (timeout) while calling epaint::TextureAtlas::options
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
    let texture_atlas = TextureAtlas::new([100000000usize, 0usize], TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::Off, font_hinting: false, subpixel_binning: false });
    let _ = texture_atlas.options();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_epaint_textureatlas_options_f7125c`).

