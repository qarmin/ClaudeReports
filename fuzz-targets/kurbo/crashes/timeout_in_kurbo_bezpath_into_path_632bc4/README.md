# timeout in kurbo::BezPath::into_path

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::BezPath::into_path`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 2366412726977858358

## Message

```
process ended abnormally (timeout) while calling kurbo::BezPath::into_path
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
    let _ = <kurbo::BezPath as kurbo::Shape>::into_path(kurbo::expand_path(Arc::new(Point::ZERO, Vec2::from_angle(1.0f64), 0.0f64, 1.0f64, 0.0f64), Diagonal2::new(0.0f64, 0.0f64), Join::Bevel, 0.0f64, 0.0f64), 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_bezpath_into_path_632bc4`).

