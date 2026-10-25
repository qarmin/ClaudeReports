# timeout in image::error::ParameterErrorKind::CicpMismatch

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::error::ParameterErrorKind::CicpMismatch`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 425044634529959146

## Message

```
process ended abnormally (timeout) while calling image::error::ParameterErrorKind::CicpMismatch
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
use image::codecs::pnm::SampleEncoding;
use image::error::ParameterErrorKind;

fn main() {
    let _ = SampleEncoding::Binary;
    let _ = <image::Rgb<u16> as image::Pixel>::to_luma_alpha(&<image::Rgb<u16> as image::Pixel>::broadcast(1000u16));
    let dynamic_image = DynamicImage::ImageRgb32F(DynamicImage::new_luma_a16(16777216u32, 1u32).to_rgb32f());
    let _ = ParameterErrorKind::CicpMismatch { expected: dynamic_image.fast_blur(-42.5f32).into_luma8().color_space(), found: <image::ImageBuffer<image::Rgba<u16>, Vec<u16>> as ::core::default::Default>::default().color_space() };
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_error_parametererrorkind_cicpmismatch_0ded85`).

