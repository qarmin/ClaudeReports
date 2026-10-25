# timeout in image::DynamicImage::from<image::DynamicImage>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::from<image::DynamicImage>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 13136319541485764929

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::from<image::DynamicImage>
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
    let _ = DynamicImage::from(DynamicImage::new_luma_a8(1u32, 65535u32).fast_blur(16777216.0f32));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_from_image_dynamicimage_98dbe0`).

