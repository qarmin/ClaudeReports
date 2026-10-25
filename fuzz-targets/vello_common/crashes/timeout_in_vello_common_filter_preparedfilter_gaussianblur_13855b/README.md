# timeout in vello_common::filter::PreparedFilter::GaussianBlur

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: timeout
- last API call: `vello_common::filter::PreparedFilter::GaussianBlur`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10917462791819582299

## Message

```
process ended abnormally (timeout) while calling vello_common::filter::PreparedFilter::GaussianBlur
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
kurbo = { version = "=0.13.1" }
peniko = { version = "=0.6.1" }
```

`src/main.rs`:

```rust
use kurbo::Cap;
use kurbo::CuspType;
use peniko::ImageFormat;
use vello_common::encode::SweepKind;
use vello_common::filter::PreparedFilter;
use vello_common::filter::gaussian_blur::GaussianBlur;
use vello_common::filter_effects::EdgeMode;
use vello_common::multi_atlas::Atlas;
use vello_common::multi_atlas::AtlasId;
use vello_common::multi_atlas::AtlasUsageStats;

fn main() {
    let _ = AtlasUsageStats { allocated_area: 1u32, total_area: 1503970760u32, allocated_count: 1u32 };
    let _ = Cap::Square;
    let _ = ImageFormat::Bgra8;
    let _ = SweepKind { start_angle: -0.25f32, inv_angle_delta: f32::NAN };
    let atlas = Atlas::new(AtlasId(1u32), 8u16, 13959u16);
    let _ = atlas.stats();
    let _ = CuspType::Loop;
    let _ = PreparedFilter::GaussianBlur(GaussianBlur::new(f32::INFINITY, EdgeMode::None));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_vello_common_filter_preparedfilter_gaussianblur_13855b`).

