# wgpu-hal/src/vulkan/conv.rs:199:22

- crate: wgpu 30.0.1
- kind: panic
- location: `wgpu-hal/src/vulkan/conv.rs:199:22`
- source: https://docs.rs/crate/wgpu-hal/30.0.1/source/src/vulkan/conv.rs#199
- last API call: `wgpu_hal::vulkan::conv::map_surface_color_space`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code: wgpu-core resolves `Auto` before configuring the surface
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
wgpu = { version = "=30.0.1" }
wgpu-hal = { version = "=30.0.1" }
wgpu-types = { version = "=30.0.1" }
```

`src/main.rs`:

```rust
use wgpu_types::SurfaceColorSpace;

fn main() {
    let _ = wgpu_hal::vulkan::conv::map_surface_color_space(SurfaceColorSpace::Auto);
}
```

Run with `cargo run` in this directory (package `repro_wgpu_hal_src_vulkan_conv_rs_199_22_ffaddf`).

## Output

```
thread 'main' (473428) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-hal-30.0.1/src/vulkan/conv.rs:199:22:
internal error: entered unreachable code: wgpu-core resolves `Auto` before configuring the surface
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: wgpu_hal::vulkan::conv::map_surface_color_space
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/wgpu-hal-30.0.1/src/vulkan/conv.rs:199:22
   3: repro_wgpu_hal_src_vulkan_conv_rs_199_22_ffaddf::main
             at ./src/main.rs:4:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
