# timeout in lopdf::Document::save_to

- crate: https://github.com/J-F-Liu/lopdf @ bebffeff2d9e54b00325014a4fda07510eac2cb1
- kind: timeout
- last API call: `lopdf::Document::save_to`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling lopdf::Document::save_to
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
lopdf = { git = "https://github.com/J-F-Liu/lopdf", rev = "bebffeff2d9e54b00325014a4fda07510eac2cb1" }
```

`src/main.rs`:

```rust
use lopdf::Document;

fn main() {
    let mut document = Document::new_from_prev(&Document::new());
    document.renumber_objects_with(2453330354u32);
    let _ = document.save_to::<Vec<u8>>(&mut include_bytes!("image_0.tiff").to_vec());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_lopdf_document_save_to_ffa94a`).

