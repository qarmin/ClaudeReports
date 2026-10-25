# parley/src/builder.rs:145:30

- crate: https://github.com/linebender/parley @ 7dc686641055e13b21d8a6757d55bf46a95a4e9e
- kind: panic
- location: `parley/src/builder.rs:145:30`
- last API call: `parley::StyleRunBuilder<[u8; 4]>::reserve`
- documented `# Panics`: false
- reproducer verified: true
- miri: clean (no undefined behavior detected)
- rfuzz seed: 47738709535597725

## Message

```
capacity overflow
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
    let mut layout_context = <parley::LayoutContext<[u8; 4]> as ::core::default::Default>::default();
    let mut font_context = FontContext::new();
    let mut style_run_builder = layout_context.style_run_builder(&mut font_context, "", 0.0f32, false);
    style_run_builder.reserve(usize::MAX, 2usize);
}
```

Run with `cargo run` in this directory (package `repro_parley_src_builder_rs_145_30_f848fa`).

## Output

```
thread 'main' (633347) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
capacity overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: alloc::raw_vec::capacity_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5
   3: alloc::raw_vec::handle_error
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:889:29
   4: <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
   5: <alloc::raw_vec::RawVecInner>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:672:17
   6: <alloc::raw_vec::RawVec<parley::resolve::ResolvedStyle<[u8; 4]>>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:341:29
   7: <alloc::vec::Vec<parley::resolve::ResolvedStyle<[u8; 4]>>>::reserve
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1461:18
   8: <parley::builder::StyleRunBuilder<[u8; 4]>>::reserve
             at /home/rafal/.cargo/git/checkouts/parley-568dff7a8f8c0912/7dc6866/parley/src/builder.rs:145:30
   9: repro_parley_src_builder_rs_145_30_f848fa::main
             at ./src/main.rs:7:23
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
