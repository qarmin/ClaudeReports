# timeout in image::DynamicImage::pixels

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::pixels`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 2481036106716601618

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::pixels
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
    let dynamic_image = DynamicImage::new_rgb8(100u32, 65535u32).unsharpen(151.0f32, 0i32);
    let _ = <image::DynamicImage as image::GenericImageView>::pixels(&dynamic_image);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_pixels_b1eab5`).

