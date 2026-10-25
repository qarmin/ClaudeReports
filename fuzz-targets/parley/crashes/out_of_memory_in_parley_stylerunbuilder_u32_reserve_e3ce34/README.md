# out of memory in parley::StyleRunBuilder<u32>::reserve

- crate: https://github.com/linebender/parley @ 7dc686641055e13b21d8a6757d55bf46a95a4e9e
- kind: out of memory
- last API call: `parley::StyleRunBuilder<u32>::reserve`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 91717791130671464

## Message

```
process ended abnormally (out of memory) while calling parley::StyleRunBuilder<u32>::reserve
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
parley = { git = "https://github.com/linebender/parley", rev = "7dc686641055e13b21d8a6757d55bf46a95a4e9e" }
```

`src/main.rs`:

```rust
use parley::FontContext;

fn main() {
    let mut layout_context = <parley::LayoutContext<u32> as ::core::default::Default>::default();
    let mut font_context = FontContext::default();
    let mut style_run_builder = layout_context.style_run_builder(&mut font_context, "", 0.0f32, false);
    style_run_builder.reserve(0usize, 1000000000usize);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_parley_stylerunbuilder_u32_reserve_e3ce34`).

## Output

```
memory allocation of 24000000000 bytes failed
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
   7: <alloc::raw_vec::RawVec<parley::resolve::StyleRun>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   8: <alloc::vec::Vec<parley::resolve::StyleRun>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   9: <parley::builder::StyleRunBuilder<u32>>::reserve
             at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parley/src/builder.rs:146:29
  10: repro_out_of_memory_in_parley_stylerunbuilder_u32_reserve_e3ce34::main
             at ./src/main.rs:7:23
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
