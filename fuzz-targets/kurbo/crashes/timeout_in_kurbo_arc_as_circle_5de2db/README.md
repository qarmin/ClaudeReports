# timeout in kurbo::Arc::as_circle

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Arc::as_circle`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14158015489758553397

## Message

```
process ended abnormally (timeout) while calling kurbo::Arc::as_circle
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
use kurbo::Circle;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Vec2;

fn main() {
    let diagonal2 = Diagonal2::new(0.0f64, 0.0f64);
    let diagonal22 = diagonal2.abs();
    let axis = Axis::Vertical;
    let point = axis.pack_point(f64::NEG_INFINITY, 0.0f64);
    let _ = <kurbo::Arc as kurbo::Shape>::as_circle(&Arc { center: kurbo::expand_path_signed(Circle::new(point, -1e300f64), <kurbo::Diagonal2 as ::core::ops::Neg>::neg(diagonal22), Join::Miter, 571.0f64, 65536.0f64).current_position().unwrap(), radii: <kurbo::Diagonal2 as ::core::ops::Mul<kurbo::Vec2>>::mul(diagonal2, Vec2 { x: 0.5f64, y: 255.0f64 }), start_angle: f64::INFINITY, sweep_angle: 0.0f64, x_rotation: 1e300f64 });
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_arc_as_circle_5de2db`).

