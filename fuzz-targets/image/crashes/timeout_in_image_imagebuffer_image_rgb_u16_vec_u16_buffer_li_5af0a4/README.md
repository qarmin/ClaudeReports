# timeout in image::ImageBuffer<image::Rgb<u16>, Vec<u16>>::buffer_like

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::ImageBuffer<image::Rgb<u16>, Vec<u16>>::buffer_like`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 16949157517595480975

## Message

```
process ended abnormally (timeout) while calling image::ImageBuffer<image::Rgb<u16>, Vec<u16>>::buffer_like
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
    let _ = <image::ImageBuffer<image::Rgb<u16>, Vec<u16>> as image::GenericImageView>::buffer_like(&DynamicImage::new_rgba16(255u32, 65535u32).blur(92.0f32).into_rgb16());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_imagebuffer_image_rgb_u16_vec_u16_buffer_li_5af0a4`).

