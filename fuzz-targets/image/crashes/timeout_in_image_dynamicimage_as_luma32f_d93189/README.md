# timeout in image::DynamicImage::as_luma32f

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::as_luma32f`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 374439503628645276

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::as_luma32f
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
    let dynamic_image = DynamicImage::new_luma8(100u32, 65535u32).unsharpen(1000.0f32, 8i32);
    let _ = dynamic_image.as_luma32f();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_as_luma32f_d93189`).

