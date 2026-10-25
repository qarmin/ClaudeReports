# timeout in image::DynamicImage::as_luma_alpha16

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::as_luma_alpha16`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10579952977590081245

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::as_luma_alpha16
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
    let dynamic_image = DynamicImage::new_rgb16(255u32, 10u32).thumbnail(65535u32, 1000000000u32);
    let _ = dynamic_image.as_luma_alpha16();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_as_luma_alpha16_e27773`).

