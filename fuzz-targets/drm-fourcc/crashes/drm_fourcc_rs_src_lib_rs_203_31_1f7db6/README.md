# drm-fourcc-rs/src/lib.rs:203:31

- crate: https://github.com/danielzfranklin/drm-fourcc-rs @ 1ad1a978a9aaa78ff0016cc7013787e4d1d68682
- kind: panic
- location: `drm-fourcc-rs/src/lib.rs:203:31`
- last API call: `drm_fourcc::UnrecognizedFourcc::string_form`
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
drm-fourcc = { git = "https://github.com/danielzfranklin/drm-fourcc-rs", rev = "1ad1a978a9aaa78ff0016cc7013787e4d1d68682" }
```

`src/main.rs`:

```rust
use drm_fourcc::UnrecognizedFourcc;

fn main() {
    let unrecognized_fourcc = UnrecognizedFourcc(2173415155u32);
    let _ = unrecognized_fourcc.string_form();
}
```

Run with `cargo run` in this directory (package `repro_drm_fourcc_rs_src_lib_rs_203_31_1f7db6`).

## Output

```
thread 'main' (1415130) panicked at /home/rafal/.cargo/git/checkouts/drm-fourcc-rs-a56117f96c475a80/1ad1a97/src/lib.rs:203:31:
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
   4: <core::option::Option<char>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: drm_fourcc::fourcc_display_form
             at /home/rafal/.cargo/git/checkouts/drm-fourcc-rs-a56117f96c475a80/1ad1a97/src/lib.rs:203:31
   6: drm_fourcc::fourcc_string_form
             at /home/rafal/.cargo/git/checkouts/drm-fourcc-rs-a56117f96c475a80/1ad1a97/src/lib.rs:195:5
   7: <drm_fourcc::UnrecognizedFourcc>::string_form
             at /home/rafal/.cargo/git/checkouts/drm-fourcc-rs-a56117f96c475a80/1ad1a97/src/lib.rs:154:9
   8: repro_drm_fourcc_rs_src_lib_rs_203_31_1f7db6::main
             at ./src/main.rs:5:33
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
