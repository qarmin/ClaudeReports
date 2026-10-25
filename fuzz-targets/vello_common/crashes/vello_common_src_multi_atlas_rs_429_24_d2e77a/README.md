# vello_common/src/multi_atlas.rs:429:24

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: panic
- location: `vello_common/src/multi_atlas.rs:429:24`
- last API call: `vello_common::multi_atlas::AtlasError::to_string`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 15758528448887769755

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::multi_atlas::AtlasError;
use vello_common::multi_atlas::AtlasId;
use vello_common::multi_atlas::AtlasLayerDiagnostics;
use vello_common::multi_atlas::AtlasSpaceDiagnostics;

fn main() {
    let atlas_error = AtlasError::AtlasLimitReached { max_atlases: 0usize, diagnostics: AtlasSpaceDiagnostics::Allocation { width: 0u16, height: 0u16, atlas_width: 0u16, atlas_height: 0u16, max_atlases: 0usize, atlases: vec![AtlasLayerDiagnostics { atlas_id: AtlasId::new(0u32), total_area: 0u64, free_area: u64::MAX, free_rectangle_count: 255usize, largest_free_width: 7u16, largest_free_height: 63490u16 }, AtlasLayerDiagnostics { atlas_id: AtlasId(1u32), total_area: 76u64, free_area: 18446744073709551614u64, free_rectangle_count: 11usize, largest_free_width: 32266u16, largest_free_height: 65534u16 }] } };
    let _ = atlas_error.to_string();
}
```

Run with `cargo run` in this directory (package `repro_vello_common_src_multi_atlas_rs_429_24_d2e77a`).

## Output

```
thread 'main' (1208403) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:206:1:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <u64 as core::iter::traits::accum::Sum>::sum::<core::iter::adapters::map::Map<core::slice::iter::Iter<vello_common::multi_atlas::AtlasLayerDiagnostics>, <vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display>::fmt::{closure#1}>>::{closure#0}
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:55:28
   4: core::iter::adapters::map::map_fold::<&vello_common::multi_atlas::AtlasLayerDiagnostics, u64, u64, <vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display>::fmt::{closure#1}, <u64 as core::iter::traits::accum::Sum>::sum<core::iter::adapters::map::Map<core::slice::iter::Iter<vel... (+167 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:88:21
   5: <core::slice::iter::Iter<vello_common::multi_atlas::AtlasLayerDiagnostics> as core::iter::traits::iterator::Iterator>::fold::<u64, core::iter::adapters::map::map_fold<&vello_common::multi_atlas::AtlasLayerDiagnostics, u64, u64, <vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Di... (+297 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/iter/macros.rs:274:27
   6: <core::iter::adapters::map::Map<core::slice::iter::Iter<vello_common::multi_atlas::AtlasLayerDiagnostics>, <vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display>::fmt::{closure#1}> as core::iter::traits::iterator::Iterator>::fold::<u64, <u64 as core::iter::traits::accum::Sum>... (+218 chars)
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/map.rs:128:19
   7: <u64 as core::iter::traits::accum::Sum>::sum::<core::iter::adapters::map::Map<core::slice::iter::Iter<vello_common::multi_atlas::AtlasLayerDiagnostics>, <vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display>::fmt::{closure#1}>>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/accum.rs:52:22
   8: <core::iter::adapters::map::Map<core::slice::iter::Iter<vello_common::multi_atlas::AtlasLayerDiagnostics>, <vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display>::fmt::{closure#1}> as core::iter::traits::iterator::Iterator>::sum::<u64>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/iterator.rs:3670:9
   9: <vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/multi_atlas.rs:519:69
  10: <&vello_common::multi_atlas::AtlasSpaceDiagnostics as core::fmt::Display>::fmt
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:2872:62
  11: <core::fmt::rt::Argument>::fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/fmt/rt.rs:152:76
  12: core::fmt::write
  13: <core::fmt::Formatter>::write_fmt
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/fmt/mod.rs:2132:13
  14: <vello_common::multi_atlas::AtlasError as core::fmt::Display>::fmt
             at /home/rafal/.cargo/git/checkouts/vello-348d5f1672e79ca5/9cbe466/vello_common/src/multi_atlas.rs:429:24
  15: <vello_common::multi_atlas::AtlasError as alloc::string::SpecToString>::spec_to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2952:9
  16: <vello_common::multi_atlas::AtlasError as alloc::string::ToString>::to_string
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/string.rs:2931:9
  17: repro_vello_common_src_multi_atlas_rs_429_24_d2e77a::main
             at ./src/main.rs:8:25
  18: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
