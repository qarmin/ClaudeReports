# read-fonts/src/tables/glyf/outline/scale.rs:394:13

- crate: https://github.com/googlefonts/fontations @ 5c480698f43ab4ff5b304de5a9204c1b5e720aec
- kind: panic
- location: `read-fonts/src/tables/glyf/outline/scale.rs:394:13`
- last API call: `read_fonts::tables::glyf::outline::Scale26Dot6::component_offset`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

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
use read_fonts::tables::glyf::outline::Scale26Dot6;
use read_fonts::tables::gvar::GlyphDelta;

fn main() {
    let _ = <read_fonts::tables::glyf::outline::Scale26Dot6 as read_fonts::tables::glyf::outline::Scale>::component_offset(&Scale26Dot6::new(None, 0u16), GlyphDelta { position: 1u16, x_delta: 65536i32, y_delta: 0i32 }.apply_scalar::<i32>(Fixed::MAX), &Transform::default(), false, Some(GlyphDelta { position: 1u16, x_delta: 1i32, y_delta: 0i32 }.apply_scalar::<font_types::Fixed>(Fixed::NEG_ONE)), false);
}
```

Run with `cargo run` in this directory (package `repro_read_fonts_src_tables_glyf_outline_scale_rs_394_13_fd5769`).

## Output

```
thread 'main' (2390257) panicked at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/glyf/outline/scale.rs:394:13:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <read_fonts::tables::glyf::outline::scale::Scale26Dot6 as read_fonts::tables::glyf::outline::scale::Scale>::component_offset
             at /home/rafal/.cargo/git/checkouts/fontations-2d2a1d0cf2f3a70a/5c48069/read-fonts/src/tables/glyf/outline/scale.rs:394:13
   4: repro_read_fonts_src_tables_glyf_outline_scale_rs_394_13_fd5769::main
             at ./src/main.rs:7:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
