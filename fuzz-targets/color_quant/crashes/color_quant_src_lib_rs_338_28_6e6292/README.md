# color_quant/src/lib.rs:338:28

- crate: https://github.com/image-rs/color_quant @ 0e2e48b82e61cc4795882e916aec64b26f1a761a
- kind: panic
- location: `color_quant/src/lib.rs:338:28`
- last API call: `color_quant::NeuQuant::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to divide by zero
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
    let _ = NeuQuant::new(0i32, 0usize, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_color_quant_src_lib_rs_338_28_6e6292`).

## Output

```
thread 'main' (444817) panicked at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:338:28:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <color_quant::NeuQuant>::learn
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:338:28
   4: <color_quant::NeuQuant>::init
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:180:14
   5: <color_quant::NeuQuant>::new
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:148:14
   6: repro_color_quant_src_lib_rs_338_28_6e6292::main
             at ./src/main.rs:4:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
