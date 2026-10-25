# timeout in kurbo::Rect::as_path_slice

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Rect::as_path_slice`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 16768126391194702226

## Message

```
process ended abnormally (timeout) while calling kurbo::Rect::as_path_slice
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
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let point = Point::new(0.0f64, 0.0f64);
    let vec2 = point.to_vec2();
    let _ = <kurbo::Rect as kurbo::Shape>::as_path_slice(&<kurbo::Arc as kurbo::Shape>::bounding_box(&Arc::new(Point::new(1.0f64, 1.0f64), Vec2::splat(f64::INFINITY).lerp(vec2, f64::MIN), 0.5f64, f64::MAX, 0.216f64)));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_rect_as_path_slice_41d505`).

