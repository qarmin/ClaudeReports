# krilla/src/content.rs:1282:39

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: panic
- location: `krilla/src/content.rs:1282:39`
- last API call: `krilla::surface::Surface::push_clip_path`
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
use krilla::document::Document;
use krilla::geom::PathBuilder;
use krilla::paint::FillRule;

fn main() {
    let mut path_builder = PathBuilder::new();
    path_builder.quad_to(0.0f32, 0.0f32, 0.0f32, 0.0f32);
    path_builder.close();
    let fill_rule = FillRule::NonZero;
    let mut document = Document::default();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.push_clip_path(&path_builder.finish().unwrap(), &fill_rule);
}
```

Run with `cargo run` in this directory (package `repro_krilla_src_content_rs_1282_39_5ba7c3`).

## Output

```
thread 'main' (251237) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1282:39:
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
   4: <core::option::Option<tiny_skia_path::Point>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: <krilla::content::ContentBuilder>::content_draw_path::<tiny_skia_path::path::PathSegmentsIter>
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:1282:39
   6: <krilla::content::ContentBuilder>::push_clip_path
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/content.rs:381:14
   7: <krilla::surface::Surface>::push_clip_path
             at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:449:27
   8: repro_krilla_src_content_rs_1282_39_5ba7c3::main
             at ./src/main.rs:13:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' (251237) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9:
assertion failed: self.push_instructions.is_empty()
stack backtrace:
   0:     0x60b535ea7061 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::libunwind::trace
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1:     0x60b535ea7061 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::trace_unsynchronized::<std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt::{closure#1}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2:     0x60b535ea7061 - std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:74:9
   3:     0x60b535ea7061 - <<std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[a9090a07a7fa7517]::fmt::Display>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:44:26
   4:     0x60b535eb914a - <core[a9090a07a7fa7517]::fmt::rt::Argument>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/fmt/rt.rs:152:76
   5:     0x60b535eb914a - core[a9090a07a7fa7517]::fmt::write
   6:     0x60b535eaa54c - core[a9090a07a7fa7517]::io::write::default_write_fmt::<std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:402:11
   7:     0x60b535eaa54c - <std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr as core[a9090a07a7fa7517]::io::write::Write>::write_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:335:13
   8:     0x60b535e90df6 - <std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:47:9
   9:     0x60b535e90df6 - std[c29fa2d9dbea2b0e]::panicking::default_hook::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:292:27
  10:     0x60b535ea34a1 - std[c29fa2d9dbea2b0e]::panicking::default_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:319:9
  11:     0x60b535ea365b - std[c29fa2d9dbea2b0e]::panicking::panic_with_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:815:13
  12:     0x60b535e90ed4 - std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:681:13
  13:     0x60b535e89fa9 - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_end_short_backtrace::<std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}, !>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:182:18
  14:     0x60b535e9163d - __rustc[9d087a5ab36ea756]::rust_begin_unwind
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
  15:     0x60b535eb98fc - core[a9090a07a7fa7517]::panicking::panic_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
  16:     0x60b535eb98c2 - core[a9090a07a7fa7517]::panicking::panic
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
  17:     0x60b535e43c62 - <krilla[c6a8bca0cb069094]::surface::Surface as core[a9090a07a7fa7517]::ops::drop::Drop>::drop
                               at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9
  18:     0x60b535e280d7 - core[a9090a07a7fa7517]::ptr::drop_glue::<krilla[c6a8bca0cb069094]::surface::Surface>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
  19:     0x60b535e1ed69 - repro_krilla_src_content_rs_1282_39_5ba7c3[25352f0f33755082]::main
                               at /home/rafal/Projekty/Rust/rust-fuzzer/targets/krilla/crashes/krilla_src_content_rs_1282_39_5ba7c3/src/main.rs:14:1
  20:     0x60b535e1efcb - <fn() as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  21:     0x60b535e1ebce - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:166:18
  22:     0x60b535e1ef01 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>::{closure#0}
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18
  23:     0x60b535ea2c09 - <&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/ops/function.rs:287:21
  24:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  25:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<i32, &dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  26:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  27:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:175:24
  28:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  29:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<isize, std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  30:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  31:     0x60b535ea2c09 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:171:5
  32:     0x60b535e1eee7 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:205:5
  33:     0x60b535e1eeae - main
  34:     0x79b13856d601 - __libc_start_call_main
                               at ./csu/../sysdeps/nptl/libc_start_call_main.h:59:16
  35:     0x79b13856d718 - __libc_start_main_impl
                               at ./csu/../csu/libc-start.c:360:3
  36:     0x60b535e1eaf5 - _start
  37:                0x0 - <unknown>
thread 'main' (251237) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```
