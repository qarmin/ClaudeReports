# timeout in kurbo::BezPath::area

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::BezPath::area`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 17637623473221849807

## Message

```
process ended abnormally (timeout) while calling kurbo::BezPath::area
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
    let point = Point::from((0.0f32, 55433.484f32));
    let _ = <kurbo::BezPath as kurbo::Shape>::area(&kurbo::expand_path(Arc { center: point, radii: Vec2::splat(f64::NAN), start_angle: f64::NEG_INFINITY, sweep_angle: 9007199254740992.0f64, x_rotation: 9007199254740992.0f64 }, <kurbo::Diagonal2 as ::core::ops::Neg>::neg(Diagonal2::new(255.0f64, 65536.0f64)), Join::Round, 0.796f64, f64::NAN));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_bezpath_area_2d9188`).

