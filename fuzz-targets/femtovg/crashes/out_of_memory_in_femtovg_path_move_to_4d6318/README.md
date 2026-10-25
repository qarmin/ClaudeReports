# out of memory in femtovg::Path::move_to

- crate: femtovg 0.27.0
- kind: out of memory
- last API call: `femtovg::Path::move_to`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling femtovg::Path::move_to
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
femtovg = { version = "=0.27.0" }
```

`src/main.rs`:

```rust
use femtovg::Paint;
use femtovg::Path;
use femtovg::Solidity;

fn main() {
    let f32_v = Paint::FONT_WEIGHT_EXTRA_BOLD;
    let mut path = Path::default();
    path.arc(0.0f32, f32_v, 10000000000.0f32, 0.0f32, 0.5f32, Solidity::default());
    let path2 = path.dashed(&[f32_v], f32_v);
    let mut path3 = path2.dashed(&[], 0.0f32);
    path3.move_to(f32_v, 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_femtovg_path_move_to_4d6318`).

## Output

```
memory allocation of 1156866432 bytes failed
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
  14: repro_out_of_memory_in_femtovg_path_move_to_4d6318::main
             at ./src/main.rs:11:11
  15: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
