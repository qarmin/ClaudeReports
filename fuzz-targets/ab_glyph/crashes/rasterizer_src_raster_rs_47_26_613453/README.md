# rasterizer/src/raster.rs:47:26

- crate: https://github.com/alexheretic/ab-glyph @ 3eb21a592819d9ed78e9a13ad7392d46c60ce39e
- kind: panic
- location: `rasterizer/src/raster.rs:47:26`
- last API call: `ab_glyph_rasterizer::Rasterizer::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ab_glyph = { git = "https://github.com/alexheretic/ab-glyph", rev = "3eb21a592819d9ed78e9a13ad7392d46c60ce39e" }
ab_glyph_rasterizer = { git = "https://github.com/alexheretic/ab-glyph", rev = "3eb21a592819d9ed78e9a13ad7392d46c60ce39e" }
```

`src/main.rs`:

```rust
use ab_glyph_rasterizer::Rasterizer;

fn main() {
    let _ = Rasterizer::new(10000000000000000000usize, 3usize);
}
```

Run with `cargo run` in this directory (package `repro_rasterizer_src_raster_rs_47_26_613453`).

## Output

```
thread 'main' (1337048) panicked at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:47:26:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <ab_glyph_rasterizer::raster::Rasterizer>::new
             at /home/rafal/.cargo/git/checkouts/ab-glyph-f1bc811571ac77d5/3eb21a5/rasterizer/src/raster.rs:47:26
   4: repro_rasterizer_src_raster_rs_47_26_613453::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
