# czkawka_core/src/tools/similar_videos/mod.rs:182:9

- crate: https://github.com/qarmin/czkawka @ eb8b91dbb4d25dff416202674ee0de84ee4b5e5c
- kind: panic
- location: `czkawka_core/src/tools/similar_videos/mod.rs:182:9`
- last API call: `czkawka_core::tools::similar_videos::SimilarVideosParameters::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: (0..=MAX_TOLERANCE).contains(&tolerance)
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
czkawka_core = { git = "https://github.com/qarmin/czkawka", rev = "eb8b91dbb4d25dff416202674ee0de84ee4b5e5c" }
```

`src/main.rs`:

```rust
use czkawka_core::tools::similar_videos::SimilarVideosParameters;

fn main() {
    let _ = SimilarVideosParameters::new(1000000000i32, false, false, 0u32, 0u32, false, 0u32, 0.0f64, 0.0f64, 0.0f64, false, 0u8, false, 0u8, false, 0.0f64, 0.0f64, 1.0f64, 1u32);
}
```

Run with `cargo run` in this directory (package `repro_czkawka_core_src_tools_similar_videos_mod_rs_182_9_ea7b63`).

## Output

```
thread 'main' (517174) panicked at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/tools/similar_videos/mod.rs:182:9:
assertion failed: (0..=MAX_TOLERANCE).contains(&tolerance)
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <czkawka_core::tools::similar_videos::SimilarVideosParameters>::new
             at /home/rafal/.cargo/git/checkouts/czkawka-c1570a3d7ff29c8c/eb8b91d/czkawka_core/src/tools/similar_videos/mod.rs:182:9
   4: repro_czkawka_core_src_tools_similar_videos_mod_rs_182_9_ea7b63::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
