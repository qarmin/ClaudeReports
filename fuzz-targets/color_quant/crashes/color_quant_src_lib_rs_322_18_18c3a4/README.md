# color_quant/src/lib.rs:322:18

- crate: https://github.com/image-rs/color_quant @ 0e2e48b82e61cc4795882e916aec64b26f1a761a
- kind: panic
- location: `color_quant/src/lib.rs:322:18`
- last API call: `color_quant::NeuQuant::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 18446744073709551615
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
    let _ = NeuQuant::new(1i32, 0usize, &vec![73u8, 73u8, 42u8, 0u8]);
}
```

Run with `cargo run` in this directory (package `repro_color_quant_src_lib_rs_322_18_18c3a4`).

## Output

```
thread 'main' (445074) panicked at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:322:18:
index out of bounds: the len is 0 but the index is 18446744073709551615
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[f64]>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:245:14
   4: <[f64] as core::ops::index::IndexMut<usize>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:32:15
   5: <alloc::vec::Vec<f64> as core::ops::index::IndexMut<usize>>::index_mut
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3943:9
   6: <color_quant::NeuQuant>::contest
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:322:18
   7: <color_quant::NeuQuant>::learn
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:368:26
   8: <color_quant::NeuQuant>::init
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:180:14
   9: <color_quant::NeuQuant>::new
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:148:14
  10: repro_color_quant_src_lib_rs_322_18_18c3a4::main
             at ./src/main.rs:4:13
  11: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
