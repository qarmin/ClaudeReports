# timeout in image::DynamicImage::from<image::ImageBuffer<image::LumaA<u8>, Vec<u8>>>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::from<image::ImageBuffer<image::LumaA<u8>, Vec<u8>>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 3224363095415602381

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::from<image::ImageBuffer<image::LumaA<u8>, Vec<u8>>>
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
    let _ = DynamicImage::from(DynamicImage::new_rgb8(256u32, 42u32).unsharpen(65536.0f32, 33554434i32).into_luma_alpha8());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_from_image_imagebuffer_image_l_39b13f`).

