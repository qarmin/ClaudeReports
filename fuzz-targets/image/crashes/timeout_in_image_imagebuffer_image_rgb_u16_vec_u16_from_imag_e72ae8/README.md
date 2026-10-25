# timeout in image::ImageBuffer<image::Rgb<u16>, Vec<u16>>::from<image::DynamicImage>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::ImageBuffer<image::Rgb<u16>, Vec<u16>>::from<image::DynamicImage>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 17697979837345323492

## Message

```
process ended abnormally (timeout) while calling image::ImageBuffer<image::Rgb<u16>, Vec<u16>>::from<image::DynamicImage>
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
use image::codecs::bmp::RowsDecoded;
use image::codecs::jpeg::PixelDensityUnit;
use image::codecs::pnm::BitmapHeader;
use image::codecs::pnm::PnmHeader;
use image::codecs::pnm::SampleEncoding;
use image::imageops::GaussianBlurParameters;

fn main() {
    let pnm_header = PnmHeader::from(BitmapHeader::from(BitmapHeader::from(BitmapHeader { encoding: SampleEncoding::Ascii, height: 1u32, width: u32::MAX })));
    let _ = pnm_header.as_graymap();
    let image_buffer = <image::ImageBuffer<image::Rgb<u16>, Vec<u16>> as ::core::convert::From<image::DynamicImage>>::from(DynamicImage::new_rgb16(55u32, 65536u32).blur_advanced(GaussianBlurParameters::new_from_sigma(256.0f32)));
    let static__vec = image_buffer.as_raw();
    let _ = <image::Luma<u16> as image::Pixel>::from_slice(static__vec);
    let _ = RowsDecoded::BottomUp { rows: 3181653221u32 };
    let mut farbfeld_encoder = <image::codecs::farbfeld::FarbfeldEncoder<Vec<u8>>>::new(include_bytes!("data_0.bin").to_vec());
    let _ = <image::codecs::farbfeld::FarbfeldEncoder<Vec<u8>> as image::ImageEncoder>::set_icc_profile(&mut farbfeld_encoder, vec![227u8, 44u8, 78u8, 252u8, 235u8, 165u8, 163u8, 176u8, 82u8, 49u8, 41u8, 68u8, 61u8, 106u8, 156u8, 187u8, 172u8, 18u8, 189u8, 53u8, 103u8, 80u8, 64u8]);
    let _ = PixelDensityUnit::Inches;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_imagebuffer_image_rgb_u16_vec_u16_from_imag_e72ae8`).

