# moxcms/src/transform.rs:191:18

- crate: moxcms 0.9.1
- kind: panic
- location: `moxcms/src/transform.rs:191:18`
- source: https://docs.rs/crate/moxcms/0.9.1/source/src/transform.rs#191
- last API call: `moxcms::Layout::r_i`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
not implemented
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { version = "=0.9.1" }
```

`src/main.rs`:

```rust
use moxcms::Layout;

fn main() {
    let layout = Layout::Inks11;
    let _ = layout.r_i();
}
```

Run with `cargo run` in this directory (package `repro_moxcms_src_transform_rs_191_18_401548`).

## Output

```
thread 'main' (2512639) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/moxcms-0.9.1/src/transform.rs:191:18:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <moxcms::transform::Layout>::r_i
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/moxcms-0.9.1/src/transform.rs:191:18
   4: repro_moxcms_src_transform_rs_191_18_401548::main
             at ./src/main.rs:5:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
