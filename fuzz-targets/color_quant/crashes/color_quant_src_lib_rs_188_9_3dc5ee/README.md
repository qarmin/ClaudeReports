# color_quant/src/lib.rs:188:9

- crate: https://github.com/image-rs/color_quant @ 0e2e48b82e61cc4795882e916aec64b26f1a761a
- kind: panic
- location: `color_quant/src/lib.rs:188:9`
- last API call: `color_quant::NeuQuant::map_pixel`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: pixel.len() == 4
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
    let neu_quant = NeuQuant::new(1i32, 1usize, &vec![]);
    neu_quant.map_pixel(&mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_color_quant_src_lib_rs_188_9_3dc5ee`).

## Output

```
thread 'main' (444669) panicked at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:188:9:
assertion failed: pixel.len() == 4
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <color_quant::NeuQuant>::map_pixel
             at /home/rafal/.cargo/git/checkouts/color_quant-f4fc608a8d16c6bc/0e2e48b/src/lib.rs:188:9
   4: repro_color_quant_src_lib_rs_188_9_3dc5ee::main
             at ./src/main.rs:5:15
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
