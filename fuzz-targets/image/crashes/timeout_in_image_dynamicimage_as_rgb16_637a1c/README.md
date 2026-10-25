# timeout in image::DynamicImage::as_rgb16

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::as_rgb16`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::as_rgb16
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

fn main() {
    let dynamic_image = DynamicImage::new_luma16(1u32, 0u32).blur(16777216.0f32);
    let _ = dynamic_image.as_rgb16();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_as_rgb16_637a1c`).

