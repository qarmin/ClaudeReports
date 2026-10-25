# rav1e/src/transform/inverse.rs:154:49

- crate: https://github.com/xiph/rav1e @ 31435de9d76fddd38f6dcc31d4014574cebb2092
- kind: panic
- location: `rav1e/src/transform/inverse.rs:154:49`
- last API call: `rav1e::bench::transform::inverse::av1_iidentity4`
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
rav1e = { git = "https://github.com/xiph/rav1e", rev = "31435de9d76fddd38f6dcc31d4014574cebb2092", features = ["aom-sys", "asm", "av-metrics", "backtrace", "bench", "binaries", "byteorder", "capi", "cc", "channel-api", "check_asm", "clap", "clap_complete", "console", "crossbeam", "dav1d-sys", "decode_test", "decode_test_dav1d", "dump_ivf", "dump_lookahead_data", "fern", "git_version", "image", "ivf", "nasm-rs", "nom", "quick_test", "scan_fmt", "serde", "serde-big-array", "serialize", "signal-hook", "signal_support", "threading", "toml", "tracing", "tracing-chrome", "tracing-subscriber", "unstable", "wasm", "wasm-bindgen", "y4m"] }
```

`src/main.rs`:

```rust
fn main() {
    rav1e::bench::transform::inverse::av1_iidentity4(&[0i32, 0i32, 0i32, -100000000i32], &mut vec![0i32, 0i32, 0i32, 0i32], 0usize);
}
```

Run with `cargo run` in this directory (package `repro_rav1e_src_transform_inverse_rs_154_49_e8cf69`).

## Output

```
thread 'main' (309355) panicked at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/inverse.rs:154:49:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: rav1e::transform::inverse::av1_iidentity4::{closure#0}
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/inverse.rs:154:49
   4: core::iter::traits::iterator::Iterator::for_each::call::<(&mut i32, &i32), rav1e::transform::inverse::av1_iidentity4::{closure#0}>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:884:29
   5: <core::iter::adapters::zip::Zip<core::slice::iter::IterMut<i32>, core::slice::iter::Iter<i32>> as core::iter::adapters::zip::ZipImpl<core::slice::iter::IterMut<i32>, core::slice::iter::Iter<i32>>>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<(&mut i32, &i32), rav1e::transform::inverse::av1_iidentity4::{closure#0}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/zip.rs:281:25
   6: <core::iter::adapters::zip::Zip<core::slice::iter::IterMut<i32>, core::slice::iter::Iter<i32>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::traits::iterator::Iterator::for_each::call<(&mut i32, &i32), rav1e::transform::inverse::av1_iidentity4::{closure#0}>::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/zip.rs:101:9
   7: <core::iter::adapters::zip::Zip<core::slice::iter::IterMut<i32>, core::slice::iter::Iter<i32>> as core::iter::traits::iterator::Iterator>::for_each::<rav1e::transform::inverse::av1_iidentity4::{closure#0}>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:887:14
   8: rav1e::transform::inverse::av1_iidentity4
             at /home/rafal/.cargo/git/checkouts/rav1e-c8400cfdd65337df/31435de/src/transform/inverse.rs:154:6
   9: repro_rav1e_src_transform_inverse_rs_154_49_e8cf69::main
             at ./src/main.rs:2:5
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
