# glow/src/lib.rs:2000:5

- crate: https://github.com/grovesNL/glow @ 62250dc64e7e531c280beea251f39aa49ba0b2f1
- kind: panic
- location: `glow/src/lib.rs:2000:5`
- last API call: `glow::compute_size`
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
glow = { git = "https://github.com/grovesNL/glow", rev = "62250dc64e7e531c280beea251f39aa49ba0b2f1" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = glow::compute_size(1000000000i32, -1000000000i32, 0u32, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_glow_src_lib_rs_2000_5_13a47c`).

## Output

```
thread 'main' (520225) panicked at /home/rafal/.cargo/git/checkouts/glow-b97fd2a55dfef1ad/62250dc/src/lib.rs:2000:5:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: glow::compute_size
             at /home/rafal/.cargo/git/checkouts/glow-b97fd2a55dfef1ad/62250dc/src/lib.rs:2000:5
   4: repro_glow_src_lib_rs_2000_5_13a47c::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
