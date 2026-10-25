# timeout in kurbo::RoundedRect::as_rounded_rect

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::RoundedRect::as_rounded_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10348944031661292294

## Message

```
process ended abnormally (timeout) while calling kurbo::RoundedRect::as_rounded_rect
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::CircleSegment;
use kurbo::Point;

fn main() {
    let circle_segment = CircleSegment::new(Point::from((0.0f32, 0.0f32)), 1.0f64, 0.0f64, -1e300f64, 0.714f64);
    let arc = circle_segment.outer_arc();
    let _ = <kurbo::RoundedRect as kurbo::Shape>::as_rounded_rect(&<kurbo::BezPath as kurbo::Shape>::as_rounded_rect(&<kurbo::Arc as kurbo::Shape>::into_path(arc, 0.0f64)).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_roundedrect_as_rounded_rect_3797b5`).

