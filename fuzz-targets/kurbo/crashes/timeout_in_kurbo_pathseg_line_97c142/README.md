# timeout in kurbo::PathSeg::Line

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::PathSeg::Line`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 6249598376174824892

## Message

```
process ended abnormally (timeout) while calling kurbo::PathSeg::Line
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
use kurbo::Arc;
use kurbo::Axis;
use kurbo::Line;
use kurbo::PathSeg;
use kurbo::Point;
use kurbo::Vec2;

fn main() {
    let axis = Axis::Horizontal;
    let axis2 = axis.cross();
    let _ = axis2.cross();
    let arc = Arc::new(Point::new(f64::MAX, -0.0f64), Vec2::new(-1.0f64, f64::NAN), f64::MAX, 2.5598864519886808e92f64, -1e300f64);
    let arc2 = arc.reversed();
    let arc3 = <kurbo::Affine as ::core::ops::Mul<kurbo::Arc>>::mul(Affine::scale(-1.5391913684270659e263f64), arc2);
    let _ = PathSeg::Line(Line::from((<kurbo::Arc as kurbo::Shape>::bounding_box(&arc3).origin(), Vec2::splat(66.8f64))));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_pathseg_line_97c142`).

