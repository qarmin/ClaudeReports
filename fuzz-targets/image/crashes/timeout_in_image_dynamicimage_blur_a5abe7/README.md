# timeout in image::DynamicImage::blur

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::blur`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10512059360691748007

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::blur
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
    let dynamic_image = DynamicImage::new_rgba8(2u32, 8u32);
    let _ = dynamic_image.blur(16777216.0f32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_blur_a5abe7`).

