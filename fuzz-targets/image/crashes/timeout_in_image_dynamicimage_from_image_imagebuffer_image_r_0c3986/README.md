# timeout in image::DynamicImage::from<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::from<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7159892374395390777

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::from<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>
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
use image::codecs::jpeg::PixelDensity;
use image::codecs::pnm::SampleEncoding;
use image::imageops::BiLevel;
use image::io::DecodedImageAttributes;
use image::metadata::CicpColorPrimaries;

fn main() {
    let cicp_color_primaries = CicpColorPrimaries::RgbB;
    let _ = cicp_color_primaries.is_supported_for_transform();
    let _ = <image::codecs::hdr::HdrEncoder<Vec<u8>>>::new(include_bytes!("image_0.tiff").to_vec());
    let _ = DecodedImageAttributes::default();
    let _ = SampleEncoding::Binary;
    let _ = PixelDensity::dpi(2048u16);
    let _ = BiLevel;
    let _ = DynamicImage::from(DynamicImage::new_rgba16(2u32, 65536u32).fast_blur(f32::MAX).to_rgba8());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_r_0c3986`).

