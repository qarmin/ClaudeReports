# out of memory in femtovg::Path::dashed

- crate: femtovg 0.27.0
- kind: out of memory
- last API call: `femtovg::Path::dashed`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling femtovg::Path::dashed
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
femtovg = { version = "=0.27.0", features = ["debug_inspector", "image", "image-loading", "serde", "swash", "textlayout", "wgpu"] }
```

`src/main.rs`:

```rust
use femtovg::FontMetrics;
use femtovg::Path;

fn main() {
    let mut path = Path::new();
    let font_metrics = FontMetrics::default();
    let f32_v = font_metrics.height();
    path.svg_arc_to(1.0f32, f32::MIN, 1.0f32, false, false, f32_v, 1.0f32);
    let _ = path.dashed(&[1.0f32], 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_femtovg_path_dashed_3fa47c`).

## Output

```
memory allocation of 2147483648 bytes failed
stack backtrace:
   0: std::alloc::rust_oom
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:436:5
   1: __rustc::__rust_alloc_error_handler
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/alloc.rs:435:1
   2: alloc::alloc::handle_alloc_error::rt_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:643:13
   3: alloc::alloc::handle_alloc_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/alloc.rs:649:9
   4: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:890:38
   5: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   6: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   7: <alloc::raw_vec::RawVec<femtovg::geometry::Position>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<femtovg::geometry::Position>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <alloc::vec::Vec<femtovg::geometry::Position>>::append_elements
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:2960:14
  10: <alloc::vec::Vec<femtovg::geometry::Position> as alloc::vec::spec_extend::SpecExtend<&femtovg::geometry::Position, core::slice::iter::Iter<femtovg::geometry::Position>>>::spec_extend
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/spec_extend.rs:56:23
  11: <alloc::vec::Vec<femtovg::geometry::Position>>::extend_from_slice
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3606:14
  12: <femtovg::path::Path>::append
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/path.rs:875:21
  13: <femtovg::path::Path>::move_to
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/path.rs:269:14
  14: femtovg::path::dash_line_segment
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/path.rs:1067:18
  15: <femtovg::path::Path>::dashed_with_tolerance
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/path.rs:200:17
  16: <femtovg::path::Path>::dashed
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/femtovg-0.27.0/src/path.rs:180:14
  17: repro_out_of_memory_in_femtovg_path_dashed_3fa47c::main
             at ./src/main.rs:9:18
  18: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
