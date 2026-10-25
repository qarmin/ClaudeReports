# timeout in image::DynamicImage::fast_blur

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::fast_blur`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 11523545254134428639

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::fast_blur
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
    let dynamic_image = DynamicImage::new_luma_a32f(1u32, 1u32);
    let dynamic_image2 = dynamic_image.grayscale();
    let dynamic_image3 = dynamic_image2.adjust_contrast(0.0f32);
    let _ = dynamic_image3.fast_blur(f32::NEG_INFINITY);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_fast_blur_834cee`).

