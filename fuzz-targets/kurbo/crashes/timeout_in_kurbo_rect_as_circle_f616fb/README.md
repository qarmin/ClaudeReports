# timeout in kurbo::Rect::as_circle

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Rect::as_circle`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 929183674840886508

## Message

```
process ended abnormally (timeout) while calling kurbo::Rect::as_circle
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
use kurbo::Axis;
use kurbo::Point;

fn main() {
    let arc = Arc::new(Point::from((0.0f32, 0.0f32)), Axis::Vertical.pack_vec2(0.0f64, 0.0f64), 0.0f64, -6.067230110266265e29f64, 0.0f64);
    let _ = <kurbo::Rect as kurbo::Shape>::as_circle(&<kurbo::Arc as kurbo::Shape>::bounding_box(&arc));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_rect_as_circle_f616fb`).

