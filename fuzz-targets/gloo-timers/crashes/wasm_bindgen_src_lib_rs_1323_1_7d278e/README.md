# wasm-bindgen/src/lib.rs:1323:1

- crate: https://github.com/rustwasm/gloo @ ebd0b8d7062c4157497eb30649afafc59c94c4b0
- kind: panic
- location: `wasm-bindgen/src/lib.rs:1323:1`
- source: https://docs.rs/crate/wasm-bindgen/0.2.129/source/src/lib.rs#1323
- last API call: `gloo_timers::future::IntervalStream::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
function not implemented on non-wasm32 targets
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
gloo-timers = { git = "https://github.com/rustwasm/gloo", rev = "ebd0b8d7062c4157497eb30649afafc59c94c4b0", features = ["futures", "futures-channel", "futures-core"] }
```

`src/main.rs`:

```rust
use gloo_timers::future::IntervalStream;

fn main() {
    let _ = IntervalStream::new(0u32);
}
```

Run with `cargo run` in this directory (package `repro_wasm_bindgen_src_lib_rs_1323_1_7d278e`).

## Output

```
thread 'main' (592454) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/lib.rs:1323:1:
function not implemented on non-wasm32 targets
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: wasm_bindgen::__wbindgen_describe
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/lib.rs:117:17
   3: wasm_bindgen::describe::inform
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/describe.rs:20:14
   4: wasm_bindgen::__rt::wbg_cast::breaks_if_inlined::<wasm_bindgen::closure::OwnedClosure<dyn core::ops::function::FnMut<(), Output = ()>, true>, wasm_bindgen::closure::JsClosure>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/rt/mod.rs:102:9
   5: wasm_bindgen::__rt::wbg_cast::<wasm_bindgen::closure::OwnedClosure<dyn core::ops::function::FnMut<(), Output = ()>, true>, wasm_bindgen::closure::JsClosure>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/rt/mod.rs:129:27
   6: <wasm_bindgen::closure::ScopedClosure<dyn core::ops::function::FnMut<(), Output = ()>>>::wrap_maybe_aborting::<true, dyn core::ops::function::FnMut<(), Output = ()>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/closure.rs:359:17
   7: <wasm_bindgen::closure::ScopedClosure<dyn core::ops::function::FnMut<(), Output = ()>>>::wrap::<dyn core::ops::function::FnMut<(), Output = ()>>
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/closure.rs:327:9
   8: <gloo_timers::callback::Interval>::new::<<gloo_timers::future::IntervalStream>::new::{closure#0}>
             at /home/rafal/.cargo/git/checkouts/gloo-bc8b576e972f6aa8/ebd0b8d/crates/timers/src/callback.rs:201:23
   9: <gloo_timers::future::IntervalStream>::new
             at /home/rafal/.cargo/git/checkouts/gloo-bc8b576e972f6aa8/ebd0b8d/crates/timers/src/future.rs:181:21
  10: repro_wasm_bindgen_src_lib_rs_1323_1_7d278e::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' (592454) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:225:5:
panic in a function that cannot unwind
stack backtrace:
   0:     0x5e8faa7246c1 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::libunwind::trace
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1:     0x5e8faa7246c1 - std[c29fa2d9dbea2b0e]::backtrace_rs::backtrace::trace_unsynchronized::<std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt::{closure#1}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2:     0x5e8faa7246c1 - std[c29fa2d9dbea2b0e]::sys::backtrace::_print_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:74:9
   3:     0x5e8faa7246c1 - <<std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[a9090a07a7fa7517]::fmt::Display>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:44:26
   4:     0x5e8faa734bfa - <core[a9090a07a7fa7517]::fmt::rt::Argument>::fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/fmt/rt.rs:152:76
   5:     0x5e8faa734bfa - core[a9090a07a7fa7517]::fmt::write
   6:     0x5e8faa727aac - core[a9090a07a7fa7517]::io::write::default_write_fmt::<std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:402:11
   7:     0x5e8faa727aac - <std[c29fa2d9dbea2b0e]::sys::stdio::unix::Stderr as core[a9090a07a7fa7517]::io::write::Write>::write_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/io/write.rs:335:13
   8:     0x5e8faa70eb46 - <std[c29fa2d9dbea2b0e]::sys::backtrace::BacktraceLock>::print
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:47:9
   9:     0x5e8faa70eb46 - std[c29fa2d9dbea2b0e]::panicking::default_hook::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:292:27
  10:     0x5e8faa7211d1 - std[c29fa2d9dbea2b0e]::panicking::default_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:319:9
  11:     0x5e8faa72138b - std[c29fa2d9dbea2b0e]::panicking::panic_with_hook
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:815:13
  12:     0x5e8faa70ec24 - std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:681:13
  13:     0x5e8faa707e19 - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_end_short_backtrace::<std[c29fa2d9dbea2b0e]::panicking::panic_handler::{closure#0}, !>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/sys/backtrace.rs:182:18
  14:     0x5e8faa70f36d - __rustc[9d087a5ab36ea756]::rust_begin_unwind
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
  15:     0x5e8faa73516d - core[a9090a07a7fa7517]::panicking::panic_nounwind_fmt::runtime
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:122:22
  16:     0x5e8faa73516d - core[a9090a07a7fa7517]::panicking::panic_nounwind_fmt
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/intrinsics/mod.rs:2636:9
  17:     0x5e8faa7350eb - core[a9090a07a7fa7517]::panicking::panic_nounwind
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:225:5
  18:     0x5e8faa735277 - core[a9090a07a7fa7517]::panicking::panic_cannot_unwind
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:350:5
  19:     0x5e8faa6fb7da - wasm_bindgen[c8a5663897463339]::__wbindgen_describe
                               at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/lib.rs:116:13
  20:     0x5e8faa6f849c - wasm_bindgen[c8a5663897463339]::describe::inform
                               at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/describe.rs:20:14
  21:     0x5e8faa6f849c - wasm_bindgen[c8a5663897463339]::__rt::wbg_cast::breaks_if_inlined::<wasm_bindgen[c8a5663897463339]::closure::OwnedClosure<dyn core[a9090a07a7fa7517]::ops::function::FnMut<(), Output = ()>, true>, wasm_bindgen[c8a5663897463339]::closure::JsClosure>
                               at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/rt/mod.rs:102:9
  22:     0x5e8faa6f818b - wasm_bindgen[c8a5663897463339]::__rt::wbg_cast::<wasm_bindgen[c8a5663897463339]::closure::OwnedClosure<dyn core[a9090a07a7fa7517]::ops::function::FnMut<(), Output = ()>, true>, wasm_bindgen[c8a5663897463339]::closure::JsClosure>
                               at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/rt/mod.rs:129:27
  23:     0x5e8faa6f7a00 - <wasm_bindgen[c8a5663897463339]::closure::ScopedClosure<dyn core[a9090a07a7fa7517]::ops::function::FnMut<(), Output = ()>>>::wrap_maybe_aborting::<true, dyn core[a9090a07a7fa7517]::ops::function::FnMut<(), Output = ()>>
                               at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/closure.rs:359:17
  24:     0x5e8faa6f7a24 - <wasm_bindgen[c8a5663897463339]::closure::ScopedClosure<dyn core[a9090a07a7fa7517]::ops::function::FnMut<(), Output = ()>>>::wrap::<dyn core[a9090a07a7fa7517]::ops::function::FnMut<(), Output = ()>>
                               at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wasm-bindgen-0.2.129/src/closure.rs:327:9
  25:     0x5e8faa6f7ac8 - <gloo_timers[30ac2774105e4d57]::callback::Interval>::new::<<gloo_timers[30ac2774105e4d57]::future::IntervalStream>::new::{closure#0}>
                               at /home/rafal/.cargo/git/checkouts/gloo-bc8b576e972f6aa8/ebd0b8d/crates/timers/src/callback.rs:201:23
  26:     0x5e8faa6f8bf3 - <gloo_timers[30ac2774105e4d57]::future::IntervalStream>::new
                               at /home/rafal/.cargo/git/checkouts/gloo-bc8b576e972f6aa8/ebd0b8d/crates/timers/src/future.rs:181:21
  27:     0x5e8faa6f785f - repro_wasm_bindgen_src_lib_rs_1323_1_7d278e[ed2d0b0d73713be7]::main
                               at /home/rafal/Projekty/Rust/rust-fuzzer/targets/gloo-timers/crashes/wasm_bindgen_src_lib_rs_1323_1_7d278e/src/main.rs:4:13
  28:     0x5e8faa6f791b - <fn() as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  29:     0x5e8faa6f783e - std[c29fa2d9dbea2b0e]::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:166:18
  30:     0x5e8faa6f79a1 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>::{closure#0}
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18
  31:     0x5e8faa720939 - <&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe as core[a9090a07a7fa7517]::ops::function::FnOnce<()>>::call_once
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/ops/function.rs:287:21
  32:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  33:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<i32, &dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  34:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<&dyn core[a9090a07a7fa7517]::ops::function::Fn<(), Output = i32> + core[a9090a07a7fa7517]::marker::Sync + core[a9090a07a7fa7517]::panic::unwind_safe::RefUnwindSafe, i32>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  35:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:175:24
  36:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::do_call::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:576:43
  37:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::panicking::catch_unwind::<isize, std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:544:19
  38:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::panic::catch_unwind::<std[c29fa2d9dbea2b0e]::rt::lang_start_internal::{closure#0}, isize>
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panic.rs:359:14
  39:     0x5e8faa720939 - std[c29fa2d9dbea2b0e]::rt::lang_start_internal
                               at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/rt.rs:171:5
  40:     0x5e8faa6f7987 - std[c29fa2d9dbea2b0e]::rt::lang_start::<()>
                               at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:205:5
  41:     0x5e8faa6f788e - main
  42:     0x7f68b5753601 - __libc_start_call_main
                               at ./csu/../sysdeps/nptl/libc_start_call_main.h:59:16
  43:     0x7f68b5753718 - __libc_start_main_impl
                               at ./csu/../csu/libc-start.c:360:3
  44:     0x5e8faa6f7765 - _start
  45:                0x0 - <unknown>
thread caused non-unwinding panic. aborting.
```
