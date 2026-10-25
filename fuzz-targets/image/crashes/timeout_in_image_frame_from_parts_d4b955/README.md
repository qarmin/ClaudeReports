# timeout in image::Frame::from_parts

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::Frame::from_parts`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7123343522239386947

## Message

```
process ended abnormally (timeout) while calling image::Frame::from_parts
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
image = { git = "https://github.com/image-rs/image", rev = "075e65d618a323e5cb48c1d8710a2eca25d690c6" }
```

`src/main.rs`:

```rust
use image::Delay;
use image::DynamicImage;
use image::Frame;

fn main() {
    let _ = Frame::from_parts(DynamicImage::new_rgba8(0u32, 2u32).blur(16777216.0f32).to_rgba8(), 0u32, u32::MAX, Frame::from_parts(<image::ImageBuffer<image::Rgba<u8>, Vec<u8>> as ::core::default::Default>::default(), 65535u32, 256u32, Delay::from_millis(84u32)).delay());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_frame_from_parts_d4b955`).

