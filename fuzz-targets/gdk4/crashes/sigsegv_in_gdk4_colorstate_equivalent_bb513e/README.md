# SIGSEGV in gdk4::ColorState::equivalent

- crate: gdk4 0.11.5
- kind: SIGSEGV
- last API call: `gdk4::ColorState::equivalent`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (SIGSEGV) while calling gdk4::ColorState::equivalent
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gdk4 = { version = "=0.11.5", features = ["gl", "v4_10", "v4_12", "v4_14", "v4_16", "v4_18", "v4_2", "v4_20", "v4_22", "v4_4", "v4_6", "v4_8"] }
```

`src/main.rs`:

```rust
use gdk4::ColorState;

fn main() {
    let color_state = ColorState::oklab();
    let _ = color_state.equivalent(&ColorState::oklch());
}
```

Run with `cargo run` in this directory (package `repro_sigsegv_in_gdk4_colorstate_equivalent_bb513e`).

