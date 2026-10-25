# color_quant/src/lib.rs:140:22

- crate: https://github.com/image-rs/color_quant @ 0e2e48b82e61cc4795882e916aec64b26f1a761a
- kind: panic
- location: `color_quant/src/lib.rs:140:22`
- last API call: `color_quant::NeuQuant::new`
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
color_quant = { git = "https://github.com/image-rs/color_quant", rev = "0e2e48b82e61cc4795882e916aec64b26f1a761a" }
```

`src/main.rs`:

```rust
use color_quant::NeuQuant;

fn main() {
    let _ = NeuQuant::new(0i32, 1000000000000000000usize, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_color_quant_src_lib_rs_140_22_38a485`).

## Output

```
thread 'main' (444780) panicked at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/alloc/src/raw_vec/mod.rs:28:5:
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
   5: <alloc::raw_vec::RawVec<color_quant::Quad<f64>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   6: <alloc::vec::Vec<color_quant::Quad<f64>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   7: <alloc::vec::Vec<color_quant::Quad<f64>>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   8: <color_quant::NeuQuant>::new
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:140:22
   9: repro_color_quant_src_lib_rs_140_22_38a485::main
             at ./src/main.rs:4:13
  10: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
