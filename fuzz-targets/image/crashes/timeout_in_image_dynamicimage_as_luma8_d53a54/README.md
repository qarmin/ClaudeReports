# timeout in image::DynamicImage::as_luma8

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::as_luma8`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10491972752114030044

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::as_luma8
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
    let dynamic_image = DynamicImage::new_rgba8(1u32, 1u32).fast_blur(-3882911600000000.0f32);
    let _ = dynamic_image.as_luma8();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_as_luma8_d53a54`).

