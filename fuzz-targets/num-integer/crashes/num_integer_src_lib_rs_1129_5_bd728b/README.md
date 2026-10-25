# num-integer/src/lib.rs:1129:5

- crate: https://github.com/rust-num/num-integer @ 765fd9b3eb9397471d41b9f613e530f6d9d6383f
- kind: panic
- location: `num-integer/src/lib.rs:1129:5`
- last API call: `num_integer::binomial[i32]`
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
num-integer = { git = "https://github.com/rust-num/num-integer", rev = "765fd9b3eb9397471d41b9f613e530f6d9d6383f" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = num_integer::binomial::<i32>(10000i32, 8i32);
}
```

Run with `cargo run` in this directory (package `repro_num_integer_src_lib_rs_1129_5_bd728b`).

## Output

```
thread 'main' (388276) panicked at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:1129:5:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i32 as core::ops::arith::Mul>::mul
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:350:45
   4: num_integer::multiply_and_divide::<i32>
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:1129:5
   5: num_integer::binomial::<i32>
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:1164:13
   6: repro_num_integer_src_lib_rs_1129_5_bd728b::main
             at ./src/main.rs:2:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
