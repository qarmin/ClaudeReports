# timeout in kurbo::expand_path_signed[Arc]

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::expand_path_signed[Arc]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 5217768614632885892

## Message

```
process ended abnormally (timeout) while calling kurbo::expand_path_signed[Arc]
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
    let _ = kurbo::expand_path_signed(CircleSegment::new(Point::from((0.0f32, 0.0f32)), 1.0f64, 1.0f64, 0.0f64, 1.0f64).outer_arc(), Diagonal2 { xx: 0.0f64, yy: 0.0f64 }, Join::Bevel, 0.0f64, 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_expand_path_signed_arc_ea8cb0`).

