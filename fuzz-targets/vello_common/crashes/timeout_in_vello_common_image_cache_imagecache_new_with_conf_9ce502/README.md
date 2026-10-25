# timeout in vello_common::image_cache::ImageCache::new_with_config

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: timeout
- last API call: `vello_common::image_cache::ImageCache::new_with_config`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 164071306093657768

## Message

```
process ended abnormally (timeout) while calling vello_common::image_cache::ImageCache::new_with_config
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
kurbo = { version = "=0.13.1" }
peniko = { version = "=0.6.1" }
vello_cpu = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use kurbo::Moments;
use kurbo::Stroke;
use kurbo::StrokeOptLevel;
use kurbo::simplify::SimplifyOptLevel;
use kurbo::simplify::SimplifyOptions;
use peniko::ImageAlphaType;
use vello_common::encode::FocalKind;
use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::pixmap::PixelMetadata;
use vello_common::pixmap::Pixmap;
use vello_cpu::RenderMode;
use vello_cpu::Resources;

fn main() {
    let simplify_options = SimplifyOptions::default();
    let simplify_options2 = simplify_options.angle_thresh(-2.9399999999999997e-26f64);
    let simplify_options3 = simplify_options2.opt_level(SimplifyOptLevel::Subdivide);
    let simplify_options4 = simplify_options3.opt_level(SimplifyOptLevel::Subdivide);
    let _ = simplify_options4.opt_level(SimplifyOptLevel::Subdivide);
    let mut image_cache = ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 8330046325053843992usize, max_atlases: 4294967296usize, atlas_size: (16384u16, 255u16), auto_grow: true, allocation_strategy: AllocationStrategy::LeastUsed });
    let image_resource = image_cache.deallocate(Resources::default().register_image(std::sync::Arc::new(Pixmap::from_parts(include_bytes!("image_0.webp").to_vec(), 16382u16, 2u16, PixelMetadata::new(ImageAlphaType::Alpha, true))))).unwrap();
    let _ = image_resource.size();
    let _ = FocalKind::OnCircle;
    let _ = image_resource.offsets();
    let _ = Stroke::new(128.0f64);
    let _ = StrokeOptLevel::Optimized;
    let _ = vello_common::filter_effects::kernels::gaussian_3x3();
    let _ = <kurbo::Moments as ::core::ops::Add<kurbo::Moments>>::add(Moments { moment_x: 0.9999999999999999f64, moment_y: 1.0f64, moment_xx: -74.3f64, moment_xy: 0.9999999999999999f64, moment_yy: 1e-310f64 }, Moments::default());
    let _ = RenderMode::OptimizeQuality;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_vello_common_image_cache_imagecache_new_with_conf_9ce502`).

## Output

```
memory allocation of 1 bytes failed
... (previous line repeated 1 more times)
skipping backtrace printing to avoid potential recursion
```
