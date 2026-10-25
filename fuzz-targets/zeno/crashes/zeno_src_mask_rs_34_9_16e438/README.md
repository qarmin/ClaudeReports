# zeno/src/mask.rs:34:9

- crate: https://github.com/dfrg/zeno @ 698399439f8035736d01de00cbcdc413b3976a18
- kind: panic
- location: `zeno/src/mask.rs:34:9`
- last API call: `zeno::Format::buffer_size`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
zeno = { git = "https://github.com/dfrg/zeno", rev = "698399439f8035736d01de00cbcdc413b3976a18" }
```

`src/main.rs`:

```rust
use zeno::Format;

fn main() {
    let format = Format::CustomSubpixel([0.0f32, 0.0f32, 0.0f32]);
    let _ = format.buffer_size(1u32, 1169106533u32);
}
```

Run with `cargo run` in this directory (package `repro_zeno_src_mask_rs_34_9_16e438`).

## Output

```
thread 'main' (538629) panicked at /home/rafal/.cargo/git/checkouts/zeno-8b21b4be6ff7cce6/6983994/src/mask.rs:34:9:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <zeno::mask::Format>::buffer_size
             at /home/rafal/.cargo/git/checkouts/zeno-8b21b4be6ff7cce6/6983994/src/mask.rs:34:9
   4: repro_zeno_src_mask_rs_34_9_16e438::main
             at ./src/main.rs:5:20
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
