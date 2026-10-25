# timeout in kurbo::expand_path[Arc]

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::expand_path[Arc]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 13187412450131097532

## Message

```
process ended abnormally (timeout) while calling kurbo::expand_path[Arc]
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
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;

fn main() {
    let circle_segment = CircleSegment { center: Point::ZERO, outer_radius: 0.0f64, inner_radius: 1.0f64, start_angle: 0.0f64, sweep_angle: f64::INFINITY };
    let _ = kurbo::expand_path(circle_segment.outer_arc(), Diagonal2 { xx: 0.0f64, yy: 0.0f64 }, Join::Bevel, 0.0f64, 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_expand_path_arc_367806`).

