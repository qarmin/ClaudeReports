# path/src/path_builder.rs:58:21

- crate: https://github.com/linebender/tiny-skia @ 5d4754777746eef0828be166896eaf482c49f8f2
- kind: panic
- location: `path/src/path_builder.rs:58:21`
- last API call: `tiny_skia_path::PathBuilder::with_capacity`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
capacity overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
tiny-skia = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
tiny-skia-path = { git = "https://github.com/linebender/tiny-skia", rev = "5d4754777746eef0828be166896eaf482c49f8f2" }
```

`src/main.rs`:

```rust
use tiny_skia_path::PathBuilder;

fn main() {
    let _ = PathBuilder::with_capacity(0usize, 4669571948208348195usize);
}
```

Run with `cargo run` in this directory (package `repro_path_src_path_builder_rs_58_21_8e5d69`).

## Output

```
thread 'main' (19443) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   4: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   5: <alloc::raw_vec::RawVec<tiny_skia_path::Point>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<tiny_skia_path::Point>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<tiny_skia_path::Point>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <tiny_skia_path::path_builder::PathBuilder>::with_capacity
             at /home/rafal/.cargo/git/checkouts/tiny-skia-4fbce405cb081ac9/5d47547/path/src/path_builder.rs:58:21
   9: repro_path_src_path_builder_rs_58_21_8e5d69::main
             at ./src/main.rs:4:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
