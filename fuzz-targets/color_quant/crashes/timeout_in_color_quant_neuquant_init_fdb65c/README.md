# timeout in color_quant::NeuQuant::init

- crate: https://github.com/image-rs/color_quant @ 0e2e48b82e61cc4795882e916aec64b26f1a761a
- kind: timeout
- last API call: `color_quant::NeuQuant::init`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling color_quant::NeuQuant::init
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
color_quant = { git = "https://github.com/image-rs/color_quant", rev = "0e2e48b82e61cc4795882e916aec64b26f1a761a" }
```

`src/main.rs`:

```rust
use color_quant::NeuQuant;

fn main() {
    let mut neu_quant = NeuQuant::new(1i32, 65536usize, &vec![]);
    neu_quant.init(&vec![]);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_color_quant_neuquant_init_fdb65c`).

