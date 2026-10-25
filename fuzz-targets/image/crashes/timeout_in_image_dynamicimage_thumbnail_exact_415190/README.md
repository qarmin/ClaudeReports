# timeout in image::DynamicImage::thumbnail_exact

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::thumbnail_exact`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 3593513911857346307

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::thumbnail_exact
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
use image::codecs::hdr::SIGNATURE;
use image::codecs::pnm::PnmSubtype;
use image::codecs::pnm::SampleEncoding;

fn main() {
    let mut tga_encoder = <image::codecs::tga::TgaEncoder<Vec<u8>>>::new(SIGNATURE.to_vec());
    let _ = <image::codecs::tga::TgaEncoder<Vec<u8>> as image::ImageEncoder>::set_icc_profile(&mut tga_encoder, vec![]);
    let _ = PnmSubtype::Graymap(SampleEncoding::Ascii);
    let dynamic_image = DynamicImage::new_rgb32f(65535u32, 24u32);
    let dynamic_image2 = dynamic_image.thumbnail_exact(26u32, 65535u32);
    let dynamic_image3 = dynamic_image2.thumbnail(65535u32, 16u32);
    let _ = dynamic_image3.huerotate(-1329250857i32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_thumbnail_exact_415190`).

