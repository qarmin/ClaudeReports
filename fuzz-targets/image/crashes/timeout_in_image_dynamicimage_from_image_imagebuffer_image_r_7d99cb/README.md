# timeout in image::DynamicImage::from<image::ImageBuffer<image::Rgb<f32>, Vec<f32>>>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::from<image::ImageBuffer<image::Rgb<f32>, Vec<f32>>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 9148330899416007244

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::from<image::ImageBuffer<image::Rgb<f32>, Vec<f32>>>
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
    let _ = DynamicImage::from(DynamicImage::new_luma_a8(1u32, 1u32).fast_blur(-2147483600.0f32).into_rgb32f());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_r_7d99cb`).

