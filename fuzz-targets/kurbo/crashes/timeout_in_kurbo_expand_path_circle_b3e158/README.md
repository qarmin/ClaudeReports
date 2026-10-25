# timeout in kurbo::expand_path[Circle]

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::expand_path[Circle]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 10726746318525169607

## Message

```
process ended abnormally (timeout) while calling kurbo::expand_path[Circle]
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Circle;
use kurbo::Diagonal2;
use kurbo::Join;
use kurbo::Point;

fn main() {
    let point = Point::default();
    let _ = kurbo::expand_path(Circle { center: point, radius: 1.0f64 }, Diagonal2::new(0.0f64, 0.0f64), Join::Bevel, 0.0f64, 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_expand_path_circle_b3e158`).

