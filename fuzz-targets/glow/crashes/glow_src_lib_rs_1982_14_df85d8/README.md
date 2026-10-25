# glow/src/lib.rs:1982:14

- crate: https://github.com/grovesNL/glow @ 62250dc64e7e531c280beea251f39aa49ba0b2f1
- kind: panic
- location: `glow/src/lib.rs:1982:14`
- last API call: `glow::components_per_format`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
unsupported format: 37067
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
glow = { git = "https://github.com/grovesNL/glow", rev = "62250dc64e7e531c280beea251f39aa49ba0b2f1" }
```

`src/main.rs`:

```rust
use glow::MAX_TESS_CONTROL_IMAGE_UNIFORMS;

fn main() {
    let u32_v = MAX_TESS_CONTROL_IMAGE_UNIFORMS;
    let _ = glow::components_per_format(u32_v);
}
```

Run with `cargo run` in this directory (package `repro_glow_src_lib_rs_1982_14_df85d8`).

## Output

```
thread 'main' (520302) panicked at /home/rafal/.cargo/git/checkouts/glow-b97fd2a55dfef1ad/62250dc/src/lib.rs:1982:14:
unsupported format: 37067
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: glow::components_per_format
             at /home/rafal/.cargo/git/checkouts/glow-b97fd2a55dfef1ad/62250dc/src/lib.rs:1982:14
   3: repro_glow_src_lib_rs_1982_14_df85d8::main
             at ./src/main.rs:5:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
