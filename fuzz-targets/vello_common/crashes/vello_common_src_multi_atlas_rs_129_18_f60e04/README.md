# vello_common/src/multi_atlas.rs:129:18

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/multi_atlas.rs:129:18`
- last API call: `vello_common::image_cache::ImageCache::new_with_config`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 14913027128261115114

## Message

```
Failed to create initial atlas: AtlasLimitReached { max_atlases: 0, diagnostics: Unavailable }
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;

fn main() {
    let _ = ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1usize, max_atlases: 0usize, atlas_size: (0u16, 0u16), auto_grow: false, allocation_strategy: AllocationStrategy::LeastUsed });
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_multi_atlas_rs_129_18_f60e04`).

## Output

```
thread 'main' (1149961) panicked at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/multi_atlas.rs:129:18:
Failed to create initial atlas: AtlasLimitReached { max_atlases: 0, diagnostics: Unavailable }
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<vello_common::multi_atlas::AtlasId, vello_common::multi_atlas::AtlasError>>::expect
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1184:23
   4: <vello_common::multi_atlas::MultiAtlasManager>::new
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/multi_atlas.rs:129:18
   5: <vello_common::image_cache::ImageCache>::new_with_config
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/image_cache.rs:72:28
   6: repro_vello_common_src_multi_atlas_rs_129_18_f60e04::main
             at ./src/main.rs:6:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
