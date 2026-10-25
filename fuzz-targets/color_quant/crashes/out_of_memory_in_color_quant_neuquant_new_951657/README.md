# out of memory in color_quant::NeuQuant::new

- crate: https://github.com/image-rs/color_quant @ 0e2e48b82e61cc4795882e916aec64b26f1a761a
- kind: out of memory
- last API call: `color_quant::NeuQuant::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (out of memory) while calling color_quant::NeuQuant::new
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
    let _ = NeuQuant::new(0i32, 1000000000usize, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_color_quant_neuquant_new_951657`).

## Output

```
memory allocation of 32000000000 bytes failed
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
   5: <alloc::raw_vec::RawVecInner>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:443:25
   6: <alloc::raw_vec::RawVec<color_quant::Quad<f64>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20
   7: <alloc::vec::Vec<color_quant::Quad<f64>>>::with_capacity_in
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:969:20
   8: <alloc::vec::Vec<color_quant::Quad<f64>>>::with_capacity
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:522:9
   9: <color_quant::NeuQuant>::new
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:140:22
  10: repro_out_of_memory_in_color_quant_neuquant_new_951657::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
