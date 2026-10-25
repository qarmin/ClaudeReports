# timeout in image::DynamicImage::ImageLuma8

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::ImageLuma8`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14239812854134286106

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::ImageLuma8
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
    let _ = DynamicImage::ImageLuma8(DynamicImage::new_rgba32f(1u32, 1u32).fast_blur(1.84e16f32).into_luma8());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_imageluma8_eed0ee`).

