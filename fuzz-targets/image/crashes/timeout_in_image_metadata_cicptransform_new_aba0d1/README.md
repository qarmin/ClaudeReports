# timeout in image::metadata::CicpTransform::new

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::metadata::CicpTransform::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 6796137357987400547

## Message

```
process ended abnormally (timeout) while calling image::metadata::CicpTransform::new
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
use image::metadata::CicpTransform;

fn main() {
    let _ = CicpTransform::new(<image::ImageBuffer<image::Rgb<f32>, Vec<f32>> as ::core::default::Default>::default().color_space(), DynamicImage::new_rgb16(10000u32, 1000u32).blur(1000.0f32).to_rgb32f().color_space());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_metadata_cicptransform_new_aba0d1`).

