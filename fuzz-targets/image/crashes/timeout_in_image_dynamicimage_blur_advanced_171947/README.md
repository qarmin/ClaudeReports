# timeout in image::DynamicImage::blur_advanced

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::blur_advanced`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5860528310244305121

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::blur_advanced
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
use image::imageops::GaussianBlurParameters;

fn main() {
    let dynamic_image = DynamicImage::new_luma16(1u32, 53293975u32);
    let _ = dynamic_image.blur_advanced(GaussianBlurParameters::new_from_kernel_size(34.0f32));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_blur_advanced_171947`).

