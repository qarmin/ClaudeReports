# timeout in image::imageops::brighten_in_place

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::imageops::brighten_in_place`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 15463024611843345342

## Message

```
process ended abnormally (timeout) while calling image::imageops::brighten_in_place
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
    let mut dynamic_image = DynamicImage::new_luma_a8(1u32, 433870799u32);
    image::imageops::brighten_in_place::<image::DynamicImage>(&mut dynamic_image, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_imageops_brighten_in_place_fdf6ea`).

