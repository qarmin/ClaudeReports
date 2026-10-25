# timeout in kurbo::TranslateScale::mul<kurbo::BezPath>

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::TranslateScale::mul<kurbo::BezPath>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7960204116227095118

## Message

```
process ended abnormally (timeout) while calling kurbo::TranslateScale::mul<kurbo::BezPath>
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
use kurbo::TranslateScale;
use kurbo::Vec2;

fn main() {
    let point = <kurbo::Point as ::core::ops::Sub<kurbo::Vec2>>::sub(Point::ZERO, Vec2::from_angle(9007199254740992.0f64).turn_90());
    let translate_scale = TranslateScale::default();
    let join = Join::Round;
    let translate_scale2 = translate_scale.inverse();
    let vec2 = <kurbo::Point as ::core::ops::Sub<kurbo::Point>>::sub(<kurbo::Vec2 as ::core::ops::Mul<f64>>::mul(Vec2::new(0.9999999999999999f64, -0.0f64).round(), 358.0f64).to_point(), point);
    let _ = <kurbo::TranslateScale as ::core::ops::Mul<kurbo::BezPath>>::mul(translate_scale2, kurbo::expand_path(Arc { center: point, radii: vec2, start_angle: f64::NAN, sweep_angle: f64::INFINITY, x_rotation: 2.220446049250313e-16f64 }, Diagonal2 { xx: 1e-310f64, yy: 255.0f64 }, join, 62.0f64, f64::MAX));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_translatescale_mul_kurbo_bezpath_dae6c3`).

