# timeout in kurbo::simplify::SimplifyBezPath::new

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::simplify::SimplifyBezPath::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 7581142553876621523

## Message

```
process ended abnormally (timeout) while calling kurbo::simplify::SimplifyBezPath::new
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Affine;
use kurbo::Circle;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::PathEl;
use kurbo::Point;
use kurbo::Vec2;
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let point = Point::from((0.0f64, 0.0f64));
    let circle = Circle::new(point, 1.0f64);
    let _ = SimplifyBezPath::new(vec![kurbo::expand_path(Circle::from(circle), Diagonal2 { xx: 4398046511103.9995f64, yy: 0.5f64 }, Join::Miter, -5.9460652595886345e-151f64, 1e-323f64).pop().unwrap(), PathEl::CurveTo(point, point, point), <kurbo::Affine as ::core::ops::Mul<kurbo::PathEl>>::mul(Affine::reflect(point, Vec2::from((1e300f64, 1e-310f64))), PathEl::ClosePath), PathEl::MoveTo(Point::from((-0.0f64, 1.5482317065146463e31f64)))]);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_simplify_simplifybezpath_new_874b01`).

