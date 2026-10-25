# timeout in kurbo::Ellipse::perimeter

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: timeout
- last API call: `kurbo::Ellipse::perimeter`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 11336074521150987131

## Message

```
process ended abnormally (timeout) while calling kurbo::Ellipse::perimeter
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::Ellipse;

fn main() {
    let _ = <kurbo::Ellipse as kurbo::Shape>::perimeter(&Ellipse::default(), f64::MIN);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_kurbo_ellipse_perimeter_e3928f`).

