# timeout in image::ImageBuffer<image::Luma<f32>, Vec<f32>>::from<image::DynamicImage>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::ImageBuffer<image::Luma<f32>, Vec<f32>>::from<image::DynamicImage>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 16871610323681636446

## Message

```
process ended abnormally (timeout) while calling image::ImageBuffer<image::Luma<f32>, Vec<f32>>::from<image::DynamicImage>
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::ColorType;
use image::ConvertColorOptions;
use image::DynamicImage;
use image::ImageError;
use image::codecs::jpeg::PixelDensity;
use image::codecs::jpeg::PixelDensityUnit;
use image::codecs::pnm::SampleEncoding;
use image::error::ImageFormatHint;
use image::error::ParameterError;
use image::error::ParameterErrorKind;
use image::io::DecoderPreparedImage;
use image::math::Rect;
use image::metadata::CicpTransferCharacteristics;

fn main() {
    let decoder_prepared_image = DecoderPreparedImage::new(56u32, 4294705150u32, ColorType::Rgb32F);
    let _ = decoder_prepared_image.total_bytes();
    let _ = PixelDensityUnit::PixelAspectRatio;
    let _ = <image::ImageBuffer<image::Rgba<u8>, Vec<u8>> as image::GenericImageView>::in_bounds(&<image::ImageBuffer<image::Luma<u8>, Vec<u8>> as ::core::default::Default>::default().expand_palette(&[(19u8, 17u8, 94u8), (255u8, 150u8, 64u8), (220u8, 8u8, 232u8), (18u8, 7u8, 8u8)], Some(254u8)), 6u32, 52u32);
    let _ = ImageFormatHint::Unknown;
    let _ = PixelDensity { density: (96u16, 8u16), unit: PixelDensityUnit::Inches };
    let _ = <image::LumaA<u8> as ::core::convert::From<[u8; 2]>>::from([14u8, 136u8]);
    let _ = SampleEncoding::Binary;
    let _ = ConvertColorOptions::default();
    let mut image_buffer = <image::ImageBuffer<image::Luma<f32>, Vec<f32>> as ::core::convert::From<image::DynamicImage>>::from(DynamicImage::new_rgb16(40u32, 67u32).blur(16777216.0f32).to_alpha_mask());
    image_buffer.set_transfer_function(CicpTransferCharacteristics::Unspecified);
    let mut image_buffer2 = <image::ImageBuffer<image::Luma<u8>, Vec<u8>>>::from_raw(1000u32, 27u32, include_bytes!("mutated_0_pal4rletrns.bmp").to_vec()).unwrap();
    let enumerate_pixels_mut_par = image_buffer2.par_enumerate_pixels_mut();
    let _ = ImageError::Parameter(ParameterError::from_kind(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch).kind()));
    let _ = <image::SubImage<String>>::new(include_str!("string_1.txt").to_string(), Rect { x: 1000u32, y: 256u32, width: 0u32, height: 2439582098u32 });
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_imagebuffer_image_luma_f32_vec_f32_from_ima_f3c3d3`).

