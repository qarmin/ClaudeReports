# timeout in image::io::DecoderPreparedImage::from<image::ImageLayout>

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::io::DecoderPreparedImage::from<image::ImageLayout>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 11742400375100403413

## Message

```
process ended abnormally (timeout) while calling image::io::DecoderPreparedImage::from<image::ImageLayout>
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
use image::ImageLayout;
use image::io::DecoderPreparedImage;

fn main() {
    let _ = DecoderPreparedImage::from(ImageLayout::empty(DynamicImage::new_rgba16(1u32, 1u32).fast_blur(f32::MIN).color()));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_io_decoderpreparedimage_from_image_imagelay_faaae2`).

