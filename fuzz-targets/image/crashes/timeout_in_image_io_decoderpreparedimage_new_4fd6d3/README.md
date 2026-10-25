# timeout in image::io::DecoderPreparedImage::new

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::io::DecoderPreparedImage::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 2967792958134544930

## Message

```
process ended abnormally (timeout) while calling image::io::DecoderPreparedImage::new
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
use image::io::DecoderPreparedImage;

fn main() {
    let _ = DecoderPreparedImage::new(0u32, 1u32, DynamicImage::new_rgb8(1u32, 1u32).fast_blur(f32::MAX).color());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_io_decoderpreparedimage_new_4fd6d3`).

