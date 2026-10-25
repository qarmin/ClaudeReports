# timeout in vello_common::filter::PreparedFilter::new

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: timeout
- last API call: `vello_common::filter::PreparedFilter::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14705255013504068889

## Message

```
process ended abnormally (timeout) while calling vello_common::filter::PreparedFilter::new
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
kurbo = { version = "=0.13.1" }
```

`src/main.rs`:

```rust
use kurbo::Affine;
use kurbo::Point;
use vello_common::filter::PreparedFilter;
use vello_common::filter_effects::EdgeMode;
use vello_common::filter_effects::Filter;
use vello_common::filter_effects::FilterPrimitive;

fn main() {
    let _ = PreparedFilter::new(&Filter::from_primitive(FilterPrimitive::GaussianBlur { std_deviation: 1.0356042e24f32, edge_mode: EdgeMode::None }), &Affine::rotate_about(0.0f64, Point { x: 0.0f64, y: 1.0f64 }));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_vello_common_filter_preparedfilter_new_be78a1`).

