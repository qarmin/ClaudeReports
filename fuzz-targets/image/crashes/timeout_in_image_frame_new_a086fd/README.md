# timeout in image::Frame::new

- crate: https://github.com/image-rs/image @ 075e65d618a323e5cb48c1d8710a2eca25d690c6
- kind: timeout
- last API call: `image::Frame::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5401639894319295517

## Message

```
process ended abnormally (timeout) while calling image::Frame::new
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
use image::Frame;

fn main() {
    let _ = Frame::new(DynamicImage::new_luma32f(1u32, 1u32).fast_blur(f32::INFINITY).into_rgba8());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_image_frame_new_a086fd`).

