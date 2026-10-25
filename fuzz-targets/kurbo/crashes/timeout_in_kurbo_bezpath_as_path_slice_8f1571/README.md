# timeout in kurbo::BezPath::as_path_slice

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::BezPath::as_path_slice`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 11160741819111826296

## Message

```
process ended abnormally (timeout) while calling kurbo::BezPath::as_path_slice
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Arc;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::BezPath as kurbo::Shape>::as_path_slice(&kurbo::expand_path_signed(Arc { center: Point::from((0.0f64, 0.0f64)), radii: Vec2 { x: 0.0f64, y: 0.0f64 }, start_angle: 0.0f64, sweep_angle: -1.1805916207174112e21f64, x_rotation: 0.0f64 }, Diagonal2::new(0.0f64, 0.0f64), Join::Miter, 0.0f64, 0.0f64));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_bezpath_as_path_slice_8f1571`).

