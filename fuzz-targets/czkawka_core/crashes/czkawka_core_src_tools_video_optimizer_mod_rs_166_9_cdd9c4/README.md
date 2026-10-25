# czkawka_core/src/tools/video_optimizer/mod.rs:166:9

- crate: https://github.com/qarmin/czkawka @ eb8b91dbb4d25dff416202674ee0de84ee4b5e5c
- kind: panic
- location: `czkawka_core/src/tools/video_optimizer/mod.rs:166:9`
- last API call: `czkawka_core::tools::video_optimizer::VideoCropParams::with_custom_params`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
black_pixel_threshold must be 0-128, got 255
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
czkawka_core = { git = "https://github.com/qarmin/czkawka", rev = "eb8b91dbb4d25dff416202674ee0de84ee4b5e5c" }
```

`src/main.rs`:

```rust
use czkawka_core::tools::similar_videos::DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE;
use czkawka_core::tools::video_optimizer::VideoCropParams;
use czkawka_core::tools::video_optimizer::VideoCroppingMechanism;

fn main() {
    let u8_v = DEFAULT_THUMBNAIL_GRID_TILES_PER_SIDE;
    let _ = VideoCropParams::with_custom_params(VideoCroppingMechanism::BlackBars, 255u8, 0u8, 0usize, 0u32, false, 0u8, false, u8_v);
}
```

Run with `cargo run` in this directory (package `repro_czkawka_core_src_tools_video_optimizer_mod_rs_166_9_cdd9c4`).

## Output

```
thread 'main' (516993) panicked at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/tools/video_optimizer/mod.rs:166:9:
black_pixel_threshold must be 0-128, got 255
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <czkawka_core::tools::video_optimizer::VideoCropParams>::with_custom_params
             at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/tools/video_optimizer/mod.rs:166:9
   3: repro_czkawka_core_src_tools_video_optimizer_mod_rs_166_9_cdd9c4::main
             at ./src/main.rs:7:13
   4: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
