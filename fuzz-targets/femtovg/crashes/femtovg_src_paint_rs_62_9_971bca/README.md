# femtovg/src/paint.rs:62:9

- crate: https://github.com/femtovg/femtovg @ 9f5ad2614e922677a467e2d6f66fecba14d3a30a
- kind: panic
- location: `femtovg/src/paint.rs:62:9`
- last API call: `femtovg::Paint::with_font_variation`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
FontVariations: too many axes (max 4)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
femtovg = { git = "https://github.com/femtovg/femtovg", rev = "9f5ad2614e922677a467e2d6f66fecba14d3a30a" }
```

`src/main.rs`:

```rust
use femtovg::Paint;

fn main() {
    let paint = Paint::elliptical_gradient_stops(0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, vec![]);
    let paint2 = paint.clone().with_font_weight(0.0f32);
    let paint3 = paint2.clone().with_font_variation(&[1u8, 0u8, 0u8, 0u8], 0.0f32);
    let paint4 = paint3.clone().with_font_italic(false);
    let _ = paint4.clone().with_font_variation(&[0u8, 0u8, 0u8, 1u8], 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_femtovg_src_paint_rs_62_9_971bca`).

## Output

```
thread 'main' (1301551) panicked at /home/rafal/.cargo/git/checkouts/femtovg-bcb3e6d7bc417141/9f5ad26/src/paint.rs:62:9:
FontVariations: too many axes (max 4)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <femtovg::paint::FontVariations>::set
             at /home/rafal/.cargo/git/checkouts/femtovg-bcb3e6d7bc417141/9f5ad26/src/paint.rs:62:9
   3: <femtovg::paint::Paint>::set_font_variation
             at /home/rafal/.cargo/git/checkouts/femtovg-bcb3e6d7bc417141/9f5ad26/src/paint.rs:1569:35
   4: <femtovg::paint::Paint>::with_font_variation
             at /home/rafal/.cargo/git/checkouts/femtovg-bcb3e6d7bc417141/9f5ad26/src/paint.rs:1577:14
   5: repro_femtovg_src_paint_rs_62_9_971bca::main
             at ./src/main.rs:8:28
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
