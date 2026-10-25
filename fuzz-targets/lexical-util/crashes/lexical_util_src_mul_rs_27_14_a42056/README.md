# lexical-util/src/mul.rs:27:14

- crate: https://github.com/Alexhuszagh/rust-lexical @ afb2efa651f42f63415a3bf3d51ced6143132b2b
- kind: panic
- location: `lexical-util/src/mul.rs:27:14`
- last API call: `lexical_util::mul::mul[u32, u8]`
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
lexical-util = { git = "https://github.com/Alexhuszagh/rust-lexical", rev = "afb2efa651f42f63415a3bf3d51ced6143132b2b" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = lexical_util::mul::mul::<u32, u8>(1000000000u32, 1000000000u32);
}
```

Run with `cargo run` in this directory (package `repro_lexical_util_src_mul_rs_27_14_a42056`).

## Output

```
thread 'main' (467706) panicked at /home/rafal/.cargo/git/checkouts/rust-lexical-b54ef1c212894bbd/afb2efa/lexical-util/src/mul.rs:27:14:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <u32 as core::ops::arith::Mul>::mul
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:350:45
   4: lexical_util::mul::mul::<u32, u8>
             at /home/rafal/.cargo/git/checkouts/rust-lexical-b54ef1c212894bbd/afb2efa/lexical-util/src/mul.rs:27:14
   5: repro_lexical_util_src_mul_rs_27_14_a42056::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
