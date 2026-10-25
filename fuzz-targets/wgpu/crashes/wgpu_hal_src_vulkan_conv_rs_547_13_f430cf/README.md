# wgpu-hal/src/vulkan/conv.rs:547:13

- crate: https://github.com/gfx-rs/wgpu @ efcf445784dd31537f09bc61bad27431de713fce
- kind: panic
- location: `wgpu-hal/src/vulkan/conv.rs:547:13`
- last API call: `wgpu_hal::vulkan::conv::map_present_mode`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code: Cannot create swapchain with Auto PresentationMode
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
wgpu = { git = "https://github.com/gfx-rs/wgpu", rev = "efcf445784dd31537f09bc61bad27431de713fce" }
wgpu-hal = { git = "https://github.com/gfx-rs/wgpu", rev = "efcf445784dd31537f09bc61bad27431de713fce" }
wgpu-types = { git = "https://github.com/gfx-rs/wgpu", rev = "efcf445784dd31537f09bc61bad27431de713fce" }
```

`src/main.rs`:

```rust
use wgpu_types::PresentMode;

fn main() {
    let _ = wgpu_hal::vulkan::conv::map_present_mode(PresentMode::AutoVsync);
}
```

Run with `cargo run` in this directory (package `repro_wgpu_hal_src_vulkan_conv_rs_547_13_f430cf`).

## Output

```
thread 'main' (2026578) panicked at /home/rafal/.cargo/git/checkouts/wgpu-045f9a3b3e40a5c0/efcf445/wgpu-hal/src/vulkan/conv.rs:547:13:
internal error: entered unreachable code: Cannot create swapchain with Auto PresentationMode
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: wgpu_hal::vulkan::conv::map_present_mode
             at /home/rafal/.cargo/git/checkouts/wgpu-045f9a3b3e40a5c0/efcf445/wgpu-hal/src/vulkan/conv.rs:547:13
   3: repro_wgpu_hal_src_vulkan_conv_rs_547_13_f430cf::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
