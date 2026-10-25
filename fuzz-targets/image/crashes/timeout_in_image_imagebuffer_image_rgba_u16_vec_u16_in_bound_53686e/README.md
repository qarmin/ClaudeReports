# timeout in image::ImageBuffer<image::Rgba<u16>, Vec<u16>>::in_bounds

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::ImageBuffer<image::Rgba<u16>, Vec<u16>>::in_bounds`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 9453955753979728416

## Message

```
process ended abnormally (timeout) while calling image::ImageBuffer<image::Rgba<u16>, Vec<u16>>::in_bounds
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
use image::Limits;
use image::codecs::avif::ColorSpace;
use image::codecs::png::CompressionType;
use image::codecs::png::FilterType;
use image::codecs::pnm::SampleEncoding;

fn main() {
    let _ = ColorSpace::Srgb;
    let _ = SampleEncoding::Ascii;
    let dynamic_image = DynamicImage::new_luma_a32f(65535u32, 1000u32);
    let dynamic_image2 = dynamic_image.adjust_contrast(0.0f32);
    let _ = dynamic_image2.adjust_contrast(0.0f32);
    let _ = <image::codecs::png::PngEncoder<Vec<u8>>>::new_with_quality(include_bytes!("image_0.png").to_vec(), CompressionType::Level(0u8), FilterType::Up);
    let _ = <image::ImageBuffer<image::Rgba<u16>, Vec<u16>> as image::GenericImageView>::in_bounds(&dynamic_image.to_rgba16(), 2303440273u32, 7u32);
    let _ = Limits::default();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_imagebuffer_image_rgba_u16_vec_u16_in_bound_53686e`).

