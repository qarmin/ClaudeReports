# timeout in image::DynamicImage::ImageRgba8

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::ImageRgba8`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 16656220240954869027

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::ImageRgba8
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
use image::SpecCompliance;
use image::error::LimitErrorKind;
use image::flat::NormalForm;

fn main() {
    let _ = SpecCompliance::Strict;
    let dynamic_image = DynamicImage::ImageRgba8(DynamicImage::new_rgba8(90u32, 62u32).fast_blur(f32::MIN).into_luma8().expand_palette(&[(202u8, 2u8, 154u8), (232u8, 2u8, 27u8), (16u8, 13u8, 8u8), (149u8, 14u8, 69u8), (0u8, 8u8, 87u8), (16u8, 100u8, 100u8)], Some(255u8)));
    let image_buffer = dynamic_image.clone().into_luma16();
    let static__vec = image_buffer.as_raw();
    let _ = <image::Rgba<u16> as image::Pixel>::from_slice(static__vec);
    let _ = LimitErrorKind::from(LimitErrorKind::InsufficientMemory);
    let _ = NormalForm::Unaliased;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_imagergba8_7ae4b9`).

