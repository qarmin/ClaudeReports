# timeout in image::DynamicImage::ImageLuma32F

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::ImageLuma32F`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5644654448954157487

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::ImageLuma32F
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
    let _ = DynamicImage::ImageLuma32F(DynamicImage::new_rgb8(1u32, 65536u32).fast_blur(16777216.0f32).to_luma32f());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_imageluma32f_49f918`).

