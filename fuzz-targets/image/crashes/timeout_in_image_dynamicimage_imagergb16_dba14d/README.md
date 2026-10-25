# timeout in image::DynamicImage::ImageRgb16

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::ImageRgb16`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5827632967461345832

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::ImageRgb16
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
    let _ = DynamicImage::ImageRgb16(DynamicImage::new_rgb8(97u32, 65535u32).unsharpen(291.0f32, 0i32).to_rgb16());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_imagergb16_dba14d`).

