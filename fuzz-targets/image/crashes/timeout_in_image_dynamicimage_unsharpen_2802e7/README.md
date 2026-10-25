# timeout in image::DynamicImage::unsharpen

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::unsharpen`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10989818845055639684

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::unsharpen
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
    let dynamic_image = DynamicImage::new_rgba8(65536u32, 8u32);
    let _ = dynamic_image.unsharpen(256.0f32, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_unsharpen_2802e7`).

