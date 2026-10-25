# font-types/src/point.rs:82:9

- crate: https://github.com/googlefonts/fontations @ 5c480698f43ab4ff5b304de5a9204c1b5e720aec
- kind: panic
- location: `font-types/src/point.rs:82:9`
- last API call: `read_fonts::tables::glyf::outline::Unscaled::component_offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 3957106670549596879

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
read-fonts = { git = "https://github.com/googlefonts/fontations", rev = "5c480698f43ab4ff5b304de5a9204c1b5e720aec" }
font-types = { git = "https://github.com/googlefonts/fontations", rev = "5c480698f43ab4ff5b304de5a9204c1b5e720aec" }
```

`src/main.rs`:

```rust
use font_types::Fixed;
use read_fonts::tables::glyf::Transform;
use read_fonts::tables::glyf::outline::Unscaled;
use read_fonts::tables::gvar::GlyphDelta;

fn main() {
    let _ = <read_fonts::tables::glyf::outline::Unscaled as read_fonts::tables::glyf::outline::Scale>::component_offset(&Unscaled, <read_fonts::tables::gvar::GlyphDelta as read_fonts::tables::variations::TupleDelta>::new(0u16, 0i32, i32::MAX).apply_scalar::<i32>(Fixed::ONE), &Transform::default(), false, Some(GlyphDelta { position: 1u16, x_delta: 0i32, y_delta: 1i32 }.apply_scalar::<font_types::Fixed>(<font_types::Fixed as ::core::ops::Neg>::neg(Fixed::NEG_ONE))), false);
}
```

Run with `cargo run` in this directory (package `repro_font_types_src_point_rs_82_9_26e6e3`).

## Output

```
thread 'main' (288597) panicked at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/font-types/src/point.rs:82:9:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i32 as core::ops::arith::AddAssign>::add_assign
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:790:51
   4: <font_types::point::Point<i32> as core::ops::arith::AddAssign>::add_assign
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/font-types/src/point.rs:82:9
   5: <read_fonts::tables::glyf::outline::scale::Unscaled as read_fonts::tables::glyf::outline::scale::Scale>::component_offset
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/glyf/outline/scale.rs:703:13
   6: repro_font_types_src_point_rs_82_9_26e6e3::main
             at ./src/main.rs:7:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
