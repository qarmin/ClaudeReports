# timeout in epaint::TextureAtlas::image

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: timeout
- last API call: `epaint::TextureAtlas::image`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 12033542858869977818

## Message

```
process ended abnormally (timeout) while calling epaint::TextureAtlas::image
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
    let texture_atlas = TextureAtlas::new([10000usize, 0usize], TextOptions { max_texture_side: 0usize, color_transfer_function: FontColorTransferFunction::LIGHT_MODE_DEFAULT, font_hinting: false, subpixel_binning: false });
    let _ = texture_atlas.image();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_epaint_textureatlas_image_423884`).

