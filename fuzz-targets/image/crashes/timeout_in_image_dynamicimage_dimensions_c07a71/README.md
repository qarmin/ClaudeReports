# timeout in image::DynamicImage::dimensions

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::DynamicImage::dimensions`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 16906566902334361310

## Message

```
process ended abnormally (timeout) while calling image::DynamicImage::dimensions
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
    let _ = <image::DynamicImage as image::GenericImageView>::dimensions(&DynamicImage::from(DynamicImage::new_rgba8(1u32, 1u32).fast_blur(1.7197081e24f32).to_rgba16()));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_dynamicimage_dimensions_c07a71`).

