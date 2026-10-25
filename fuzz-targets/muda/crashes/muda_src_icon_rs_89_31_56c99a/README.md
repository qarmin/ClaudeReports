# muda/src/icon.rs:89:31

- crate: https://github.com/tauri-apps/muda @ 33dfdf7c6daceed1dddd73b658e0e4c912685f36
- kind: panic
- location: `muda/src/icon.rs:89:31`
- last API call: `muda::Icon::from_rgba`
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
muda = { git = "https://github.com/tauri-apps/muda", rev = "33dfdf7c6daceed1dddd73b658e0e4c912685f36", features = ["common-controls-v6", "gtk3", "gtk4", "serde", "snapshot"] }
```

`src/main.rs`:

```rust
use muda::Icon;

fn main() {
    let _ = Icon::from_rgba(vec![], 10000u32, 1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_muda_src_icon_rs_89_31_56c99a`).

## Output

```
thread 'main' (516760) panicked at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/icon.rs:89:31:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <muda::icon::RgbaIcon>::from_rgba
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/icon.rs:89:31
   4: <muda::icon::Icon>::from_rgba
             at /home/rafal/.cargo/git/checkouts/muda-7190a70c65b16524/33dfdf7/src/icon.rs:137:24
   5: repro_muda_src_icon_rs_89_31_56c99a::main
             at ./src/main.rs:4:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
