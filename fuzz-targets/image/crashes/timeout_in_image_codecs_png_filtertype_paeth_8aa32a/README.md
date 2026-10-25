# timeout in image::codecs::png::FilterType::Paeth

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::codecs::png::FilterType::Paeth`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7510173502786653763

## Message

```
process ended abnormally (timeout) while calling image::codecs::png::FilterType::Paeth
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
use image::codecs::bmp::RowsDecoded;
use image::codecs::png::FilterType;
use image::error::ImageFormatHint;
use image::error::UnsupportedErrorKind;
use image::imageops::GaussianBlurParameters;

fn main() {
    let gaussian_blur_parameters = GaussianBlurParameters::SMOOTHING_3;
    let mut limits = Limits::default();
    limits.free_usize(256usize);
    let mut luma = <image::Luma<u8> as ::core::convert::From<[u8; 1]>>::from([254u8]);
    <image::Luma<u8> as image::Pixel>::apply2::<fn(u8, u8) -> u8>(&mut luma, &<image::Luma<u8> as image::Pixel>::to_luma(&<image::Luma<u8> as ::core::convert::From<[u8; 1]>>::from([7u8])), |_, _| 1 as u8);
    let _ = UnsupportedErrorKind::Format(ImageFormatHint::Unknown);
    let dynamic_image = DynamicImage::from(DynamicImage::from(DynamicImage::new_luma_a8(1000u32, 65535u32).to_luma_alpha32f()));
    let dynamic_image2 = dynamic_image.blur_advanced(gaussian_blur_parameters);
    let dynamic_image3 = dynamic_image2.rotate270();
    let _ = dynamic_image3.as_rgb32f();
    let image_buffer = dynamic_image2.as_rgba16().unwrap();
    let static__vec = image_buffer.as_raw();
    let _ = <image::Rgba<u16> as image::Pixel>::from_slice(static__vec);
    let _ = RowsDecoded::BottomUp { rows: 4294967294u32 };
    let image_buffer2 = dynamic_image2.to_rgba32f();
    let static__vec2 = image_buffer2.as_raw();
    let _ = <image::Rgba<f32> as image::Pixel>::from_slice(static__vec2);
    let _ = FilterType::Paeth;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_codecs_png_filtertype_paeth_8aa32a`).

