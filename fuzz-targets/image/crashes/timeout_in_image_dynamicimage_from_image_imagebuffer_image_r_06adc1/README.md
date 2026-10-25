# timeout in image::DynamicImage::from<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::from<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7168004445808919907

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::from<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>>
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
    let _ = DynamicImage::from(DynamicImage::new_rgb32f(1u32, 1u32).fast_blur(-20200000000000.0f32).to_rgb8());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_r_06adc1`).

