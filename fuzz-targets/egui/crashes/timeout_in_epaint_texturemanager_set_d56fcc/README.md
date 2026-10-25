# timeout in epaint::TextureManager::set

- crate: https://github.com/emilk/egui @ 86099aca1276fe786921992c8d5a68914ee79bec
- kind: timeout
- last API call: `epaint::TextureManager::set`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 16974774762924065563

## Message

```
process ended abnormally (timeout) while calling epaint::TextureManager::set
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
use epaint::TextOptions;
use epaint::TextureAtlas;
use epaint::TextureId;
use epaint::TextureManager;

fn main() {
    let mut texture_manager = TextureManager::default();
    texture_manager.set(TextureId::default(), TextureAtlas::new([1000000000usize, 0usize], TextOptions::default()).take_delta().unwrap());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_epaint_texturemanager_set_d56fcc`).

