# timeout in tiny_skia::Pixmap::encode_png

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: timeout
- last API call: `tiny_skia::Pixmap::encode_png`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling tiny_skia::Pixmap::encode_png
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiny-skia = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
```

`src/main.rs`:

```rust
use tiny_skia::Pixmap;

fn main() {
    let pixmap = Pixmap::new(1u32, 100000000u32).unwrap();
    let _ = pixmap.encode_png();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_tiny_skia_pixmap_encode_png_dd804c`).

