# core/src/ptr/mod.rs:652:9

- crate: https://github.com/etemesi254/zune-image @ b39e0123084027389e565328c0fea8d0f721985e
- kind: panic
- location: `core/src/ptr/mod.rs:652:9`
- last API call: `zune_image::image::Image::fill[u16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
unsafe precondition(s) violated: ptr::copy requires that both pointer arguments are aligned and non-null

This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zune-image = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e", default-features = false }
zune-core = { git = "https://github.com/etemesi254/zune-image", rev = "b39e0123084027389e565328c0fea8d0f721985e" }
```

`src/main.rs`:

```rust
use zune_core::colorspace::ColorSpace;
use zune_image::image::Image;
use zune_image::image::MAX_CHANNELS;

fn main() {
    let usize_v = MAX_CHANNELS;
    let _ = Image::fill::<u16>(0u16, ColorSpace::MultiBand(std::num::NonZero::new(10000u32).unwrap()), usize_v, 65536usize);
}
```

Run with `cargo run` in this directory (package `repro_core_src_ptr_mod_rs_652_9_98b010`).

## Output

```
thread 'main' (3374254) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:652:9:
unsafe precondition(s) violated: ptr::copy requires that both pointer arguments are aligned and non-null
This indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.
stack backtrace:
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread caused non-unwinding panic. aborting.
```
