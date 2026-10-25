# timeout in image::imageops::resize

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::imageops::resize`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 12380942596055812928

## Message

```
process ended abnormally (timeout) while calling image::imageops::resize
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::DynamicImage;
use image::imageops::FilterType;

fn main() {
    let _ = image::imageops::resize::<image::DynamicImage>(&DynamicImage::new_rgb8(1u32, 1u32), 1000000000u32, 0u32, FilterType::Nearest);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_imageops_resize_db5e3b`).

