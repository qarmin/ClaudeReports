# timeout in kurbo::Rect::perimeter

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Rect::perimeter`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10021339674335295923

## Message

```
process ended abnormally (timeout) while calling kurbo::Rect::perimeter
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
    let _ = <kurbo::Rect as kurbo::Shape>::perimeter(&<kurbo::Arc as kurbo::Shape>::bounding_box(&Arc::new(Point { x: 0.0f64, y: 0.0f64 }, Vec2 { x: f64::INFINITY, y: 0.0f64 }, 0.0f64, 1.0f64, 0.0f64)), 1.0f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_rect_perimeter_436d7f`).

