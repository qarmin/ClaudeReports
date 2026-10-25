# krilla/src/content.rs:384:18

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/content.rs:384:18`
- last API call: `krilla::surface::Surface::draw_glyphs`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
krilla = { git = "https://github.com/LaurenzV/krilla", rev = "5eb4e1fb24a24ce78b533c23b080969917530c32" }
```

`src/main.rs`:

```rust
use krilla::Data;
use krilla::SerializeSettings;
use krilla::document::Document;
use krilla::geom::Point;
use krilla::text::Font;
use krilla::text::GlyphId;
use krilla::text::KrillaGlyph;

fn main() {
    let mut document = Document::new_with(SerializeSettings::default());
    let mut page = document.start_page();
    let f32_v = <krilla::text::KrillaGlyph as krilla::text::Glyph>::y_advance(&KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0usize..0usize, None), 0.0f32);
    let mut surface = page.surface();
    surface.draw_glyphs(Point::default(), &[KrillaGlyph::new(GlyphId::new(0u32), 0.0f32, 0.0f32, f32_v, f32::NAN, 0usize..0usize, None), KrillaGlyph { glyph_id: GlyphId::new(10u32), text_range: 0usize..0usize, x_advance: 0.0f32, x_offset: 0.0f32, y_offset: 0.0f32, y_advance: 0.0f32, location: None }], Font::new_variable(Data::from(std::sync::Arc::new(include_bytes!("data_0.bin").to_vec())), 0u32, &[]).unwrap(), "", 0.0f32, true);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_content_rs_384_18_41beb6`).

## Output

```
thread 'main' (251453) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:384:18:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<tiny_skia_path::path::Path>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <krilla::content::ContentBuilder>::push_clip_path
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:384:18
   6: <krilla::surface::Surface>::push_clip_path
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:449:27
   7: krilla::text::glyph::colr::interpret
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/colr.rs:82:29
   8: krilla::text::glyph::colr::draw_glyph
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/colr.rs:54:5
   9: krilla::text::glyph::draw_color_glyph
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/mod.rs:48:17
  10: krilla::text::glyph::draw_glyph
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/text/glyph/mod.rs:84:5
  11: <krilla::surface::Surface>::outline_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:278:13
  12: <krilla::surface::Surface>::draw_glyphs::<krilla::text::glyph::KrillaGlyph>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:307:18
  13: repro_krilla_src_content_rs_384_18_41beb6::main
             at ./src/main.rs:14:13
  14: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' (251453) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9:
assertion failed: self.push_instructions.is_empty()
stack backtrace:
   0:     0x5a145ba4dee1 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::libunwind::trace
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1:     0x5a145ba4dee1 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::trace_unsynchronized::<std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt::{closure#1}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2:     0x5a145ba4dee1 - std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:74:9
   3:     0x5a145ba4dee1 - <<std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[a9090a07a7fa7517]::fmt::Display>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:44:26
   4:     0x5a145ba60cea - <core[a9090a07a7fa7517]::fmt::rt::Argument>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/fmt/rt.rs:152:76
   5:     0x5a145ba60cea - core[a9090a07a7fa7517]::fmt::write
   6:     0x5a145ba513cc - core[a9090a07a7fa7517]::io::write::default_write_fmt::<std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:402:11
   7:     0x5a145ba513cc - <std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr as core[a9090a07a7fa7517]::io::write::Write>::write_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:335:13
   8:     0x5a145ba37af6 - <std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:47:9
   9:     0x5a145ba37af6 - std[c29fa2d9dbea2b0e]::panicking::default_hook::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:292:27
  10:     0x5a145ba4a301 - std[c29fa2d9dbea2b0e]::panicking::default_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:319:9
  11:     0x5a145ba4a4bb - std[c29fa2d9dbea2b0e]::panicking::panic_with_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:815:13
  12:     0x5a145ba37bd4 - std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:681:13
  13:     0x5a145ba30ca9 - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_end_short_backtrace::<std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}, !>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:182:18
  14:     0x5a145ba3833d - __rustc[9d087a5ab36ea756]::rust_begin_unwind
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
  15:     0x5a145ba614cc - core[a9090a07a7fa7517]::panicking::panic_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
  16:     0x5a145ba61492 - core[a9090a07a7fa7517]::panicking::panic
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
  17:     0x5a145b6da082 - <krilla[c6a8bca0cb069094]::surface::Surface as core[a9090a07a7fa7517]::ops::drop::Drop>::drop
                               at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9
  18:     0x5a145b637667 - core[a9090a07a7fa7517]::ptr::drop_glue::<krilla[c6a8bca0cb069094]::surface::Surface>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
  19:     0x5a145b6185c3 - repro_krilla_src_content_rs_384_18_41beb6[e25afa81fc0489d0]::main
                               at /home/rafal/Projekty/Rust/rust-fuzzer/targets/krilla/crashes/krilla_src_content_rs_384_18_41beb6/src/main.rs:15:1
  20:     0x5a145b61844b - <fn() as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  21:     0x5a145b61839e - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:166:18
  22:     0x5a145b6188f1 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>::{closure#0}
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18
  23:     0x5a145ba49a69 - <&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/ops/function.rs:287:21
  24:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  25:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<i32, &dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  26:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  27:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:175:24
  28:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  29:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<isize, std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  30:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  31:     0x5a145ba49a69 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:171:5
  32:     0x5a145b6188d7 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:205:5
  33:     0x5a145b61888e - main
  34:     0x7d620ad9f601 - __libc_start_call_main
                               at ./csu/../sysdeps/nptl/libc_start_call_main.h:59:16
  35:     0x7d620ad9f718 - __libc_start_main_impl
                               at ./csu/../csu/libc-start.c:360:3
  36:     0x5a145b6182c5 - _start
  37:                0x0 - <unknown>
thread 'main' (251453) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```
