# timeout in image::DynamicImage::as_rgba32f

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::as_rgba32f`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14861616571228136656

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::as_rgba32f
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
    let dynamic_image = DynamicImage::new_luma16(1u32, 1u32).fast_blur(f32::NEG_INFINITY);
    let _ = dynamic_image.as_rgba32f();
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_as_rgba32f_a9593a`).

