# timeout in kurbo::BezPath::as_rect

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::BezPath::as_rect`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 12924990979081638512

## Message

```
process ended abnormally (timeout) while calling kurbo::BezPath::as_rect
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
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;

fn main() {
    let affine = Affine::skew(0.0f64, 2.5699999999999997e-14f64);
    let axis = Axis::Horizontal;
    let _ = <kurbo::BezPath as kurbo::Shape>::as_rect(&kurbo::expand_path(<kurbo::Affine as ::core::ops::Mul<kurbo::Arc>>::mul(affine, Arc::new(Point::from((f64::MIN, 430.0f64)), axis.pack_vec2(f64::MIN, f64::NAN), 0.5f64, -1.1805916207174113e21f64, 0.0f64)), Diagonal2::new(-20.5f64, f64::NEG_INFINITY), Join::Bevel, 65536.0f64, 1e-310f64));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_bezpath_as_rect_6910da`).

