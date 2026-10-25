# timeout in kurbo::Arc::perimeter

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Arc::perimeter`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 14007048739356740579

## Message

```
process ended abnormally (timeout) while calling kurbo::Arc::perimeter
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
use kurbo::Vec2;

fn main() {
    let _ = <kurbo::Arc as kurbo::Shape>::perimeter(&Arc { center: Axis::Vertical.pack_vec2(0.0f64, 0.0f64).to_point(), radii: Vec2::splat(f64::INFINITY), start_angle: 0.0f64, sweep_angle: 1.0f64, x_rotation: 0.0f64 }, 2.220446049250313e-16f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_arc_perimeter_ba15d1`).

