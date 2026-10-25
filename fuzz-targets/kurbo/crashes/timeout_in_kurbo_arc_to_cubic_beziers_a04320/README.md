# timeout in kurbo::Arc::to_cubic_beziers

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Arc::to_cubic_beziers`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7545314356801774914

## Message

```
process ended abnormally (timeout) while calling kurbo::Arc::to_cubic_beziers
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
    let circle_segment = CircleSegment { center: Point::from((0.0f64, 0.0f64)), outer_radius: 1.0f64, inner_radius: f64::NAN, start_angle: 0.384f64, sweep_angle: 42.2f64 };
    let arc = circle_segment.outer_arc();
    arc.to_cubic_beziers::<fn(kurbo::Point, kurbo::Point, kurbo::Point)>(1e-310f64, |_, _, _| ());
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_arc_to_cubic_beziers_a04320`).

