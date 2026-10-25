# contract: EqReflexive for krilla::paint::Pattern

- crate: https://github.com/LaurenzV/krilla @ 5eb4e1fb24a24ce78b533c23b080969917530c32
- kind: contract
- location: `./csu/../sysdeps/nptl/libc_start_call_main.h:59:16`
- last API call: `krilla::paint::Pattern {literal}`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
rfuzz contract violated: EqReflexive for krilla::paint::Pattern
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
use krilla::geom::Transform;
use krilla::paint::Pattern;

fn main() {
    let mut document = Document::new();
    let mut page = document.start_page();
    let mut surface = page.surface();
    surface.push_transform(&Transform::from_scale(0.0f32, 0.0f32).invert().unwrap());
    let transform = surface.ctm();
    let mut stream_builder = surface.stream_builder();
    let mut surface2 = stream_builder.surface();
    let stream_builder2 = surface2.stream_builder();
    let stream = stream_builder2.finish();
    let pattern = Pattern { stream: stream.clone(), transform: transform, width: 0.0f32, height: 0.0f32 };
    assert!(pattern == pattern, "Eq is not reflexive");
}
```

Run with `cargo run` in this directory (package `repro_contract_eqreflexive_for_krilla_paint_pattern_58e221`).

## Output

```
thread 'main' (251124) panicked at src/main.rs:16:5:
Eq is not reflexive
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: repro_contract_eqreflexive_for_krilla_paint_pattern_58e221::main
             at ./src/main.rs:16:5
   3: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' (251124) panicked at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9:
assertion failed: self.push_instructions.is_empty()
stack backtrace:
   0:     0x607734e99641 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::libunwind::trace
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1:     0x607734e99641 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::trace_unsynchronized::<std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt::{closure#1}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2:     0x607734e99641 - std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:74:9
   3:     0x607734e99641 - <<std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[a9090a07a7fa7517]::fmt::Display>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:44:26
   4:     0x607734eaa53a - <core[a9090a07a7fa7517]::fmt::rt::Argument>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/fmt/rt.rs:152:76
   5:     0x607734eaa53a - core[a9090a07a7fa7517]::fmt::write
   6:     0x607734e9cb2c - core[a9090a07a7fa7517]::io::write::default_write_fmt::<std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:402:11
   7:     0x607734e9cb2c - <std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr as core[a9090a07a7fa7517]::io::write::Write>::write_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:335:13
   8:     0x607734e833d6 - <std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:47:9
   9:     0x607734e833d6 - std[c29fa2d9dbea2b0e]::panicking::default_hook::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:292:27
  10:     0x607734e95a81 - std[c29fa2d9dbea2b0e]::panicking::default_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:319:9
  11:     0x607734e95c3b - std[c29fa2d9dbea2b0e]::panicking::panic_with_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:815:13
  12:     0x607734e834b4 - std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:681:13
  13:     0x607734e7c589 - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_end_short_backtrace::<std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}, !>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:182:18
  14:     0x607734e83c1d - __rustc[9d087a5ab36ea756]::rust_begin_unwind
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
  15:     0x607734eaacec - core[a9090a07a7fa7517]::panicking::panic_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
  16:     0x607734eaacb2 - core[a9090a07a7fa7517]::panicking::panic
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
  17:     0x607734e403d2 - <krilla[c6a8bca0cb069094]::surface::Surface as core[a9090a07a7fa7517]::ops::drop::Drop>::drop
                               at /home/rafal/.cargo/git/checkouts/krilla-dfa083154a81f6b2/5eb4e1f/crates/krilla/src/surface.rs:644:9
  18:     0x607734e278a7 - core[a9090a07a7fa7517]::ptr::drop_glue::<krilla[c6a8bca0cb069094]::surface::Surface>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ptr/mod.rs:847:1
  19:     0x607734e1d274 - repro_contract_eqreflexive_for_krilla_paint_pattern_58e221[ab3270cb294107a1]::main
                               at /home/rafal/Projekty/Rust/rust-fuzzer/targets/krilla/crashes/contract_eqreflexive_for_krilla_paint_pattern_58e221/src/main.rs:17:1
  20:     0x607734e1d73b - <fn() as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  21:     0x607734e1d68e - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:166:18
  22:     0x607734e1cca1 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>::{closure#0}
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18
  23:     0x607734e951e9 - <&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/ops/function.rs:287:21
  24:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  25:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<i32, &dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  26:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  27:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:175:24
  28:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  29:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<isize, std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  30:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  31:     0x607734e951e9 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:171:5
  32:     0x607734e1cc87 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:205:5
  33:     0x607734e1d5ee - main
  34:     0x7cd3d3e61601 - __libc_start_call_main
                               at ./csu/../sysdeps/nptl/libc_start_call_main.h:59:16
  35:     0x7cd3d3e61718 - __libc_start_main_impl
                               at ./csu/../csu/libc-start.c:360:3
  36:     0x607734e1c735 - _start
  37:                0x0 - <unknown>
thread 'main' (251124) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```
