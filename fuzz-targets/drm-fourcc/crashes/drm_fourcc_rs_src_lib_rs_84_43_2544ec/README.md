# drm-fourcc-rs/src/lib.rs:84:43

- crate: https://github.com/danielzfranklin/drm-fourcc-rs @ 1ad1a978a9aaa78ff0016cc7013787e4d1d68682
- kind: panic
- location: `drm-fourcc-rs/src/lib.rs:84:43`
- last API call: `drm_fourcc::DrmFourcc::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Must be valid fourcc
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
drm-fourcc = { git = "https://github.com/danielzfranklin/drm-fourcc-rs", rev = "1ad1a978a9aaa78ff0016cc7013787e4d1d68682" }
```

`src/main.rs`:

```rust
use drm_fourcc::DrmFourcc;

fn main() {
    let drm_fourcc = DrmFourcc::R16f;
    let _ = drm_fourcc.to_string();
}
```

Run with `cargo run` in this directory (package `repro_drm_fourcc_rs_src_lib_rs_84_43_2544ec`).

## Output

```
thread 'main' (1415183) panicked at /home/rafal/.cargo/git/checkouts/drm-fourcc-rs-a56117f96c475a80/1ad1a97/src/lib.rs:84:43:
Must be valid fourcc
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:259:5
   3: core::option::expect_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2257:5
   4: <core::option::Option<drm_fourcc::fourcc_display_form::FormatFourccRaw>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:969:21
   5: <drm_fourcc::as_enum::DrmFourcc>::display_form
             at /home/rafal/.cargo/git/checkouts/drm-fourcc-rs-a56117f96c475a80/1ad1a97/src/lib.rs:84:43
   6: <drm_fourcc::as_enum::DrmFourcc as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/drm-fourcc-rs-a56117f96c475a80/1ad1a97/src/lib.rs:98:28
   7: <drm_fourcc::as_enum::DrmFourcc as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
   8: <drm_fourcc::as_enum::DrmFourcc as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
   9: repro_drm_fourcc_rs_src_lib_rs_84_43_2544ec::main
             at ./src/main.rs:5:24
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
