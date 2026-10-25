# timeout in vello_common::filter::gaussian_blur::GaussianBlur::new

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: timeout
- last API call: `vello_common::filter::gaussian_blur::GaussianBlur::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 15117242373428521977

## Message

```
process ended abnormally (timeout) while calling vello_common::filter::gaussian_blur::GaussianBlur::new
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::filter::gaussian_blur::GaussianBlur;
use vello_common::filter_effects::EdgeMode;

fn main() {
    let _ = GaussianBlur::new(1e30f32, EdgeMode::None);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_vello_common_filter_gaussian_blur_gaussianblur_ne_5ee26d`).

