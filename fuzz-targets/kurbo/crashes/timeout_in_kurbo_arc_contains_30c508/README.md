# timeout in kurbo::Arc::contains

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Arc::contains`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7735191287761970360

## Message

```
process ended abnormally (timeout) while calling kurbo::Arc::contains
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
    let _ = <kurbo::Arc as kurbo::Shape>::contains(&Arc { center: Point::default(), radii: Vec2::new(0.0f64, 0.0f64), start_angle: 0.0f64, sweep_angle: f64::MIN, x_rotation: 0.0f64 }, Point::ORIGIN);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_arc_contains_30c508`).

