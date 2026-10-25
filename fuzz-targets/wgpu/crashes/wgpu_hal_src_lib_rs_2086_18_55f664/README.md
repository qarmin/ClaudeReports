# wgpu-hal/src/lib.rs:2086:18

- crate: https://github.com/gfx-rs/wgpu @ efcf445784dd31537f09bc61bad27431de713fce
- kind: panic
- location: `wgpu-hal/src/lib.rs:2086:18`
- last API call: `wgpu_hal::FormatAspects::map`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
wgpu = { git = "https://github.com/gfx-rs/wgpu", rev = "efcf445784dd31537f09bc61bad27431de713fce" }
wgpu-hal = { git = "https://github.com/gfx-rs/wgpu", rev = "efcf445784dd31537f09bc61bad27431de713fce" }
```

`src/main.rs`:

```rust
use wgpu_hal::FormatAspects;

fn main() {
    let _ = FormatAspects::all().map();
}
```

Run with `cargo run` in this directory (package `repro_wgpu_hal_src_lib_rs_2086_18_55f664`).

## Output

```
thread 'main' (2028923) panicked at /home/rafal/.cargo/git/checkouts/wgpu-045f9a3b3e40a5c0/efcf445/wgpu-hal/src/lib.rs:2086:18:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <wgpu_hal::FormatAspects>::map
             at /home/rafal/.cargo/git/checkouts/wgpu-045f9a3b3e40a5c0/efcf445/wgpu-hal/src/lib.rs:2086:18
   4: repro_wgpu_hal_src_lib_rs_2086_18_55f664::main
             at ./src/main.rs:4:34
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
