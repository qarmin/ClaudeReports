# num-integer/src/lib.rs:552:22

- crate: https://github.com/rust-num/num-integer @ 765fd9b3eb9397471d41b9f613e530f6d9d6383f
- kind: panic
- location: `num-integer/src/lib.rs:552:22`
- last API call: `num_integer::lcm[i64]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to negate with overflow
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
    let _ = num_integer::lcm::<i64>(i64::MIN, 0i64);
}
```

Run with `cargo run` in this directory (package `repro_num_integer_src_lib_rs_552_22_cc4487`).

## Output

```
thread 'main' (388585) panicked at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/mod.rs:454:5:
attempt to negate with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_neg_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i64>::abs
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/int_macros.rs:3635:17
   4: <i64 as num_integer::Integer>::gcd
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:498:36
   5: <i64 as num_integer::Integer>::gcd_lcm
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:562:32
   6: <i64 as num_integer::Integer>::lcm
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:552:22
   7: num_integer::lcm::<i64>
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:428:7
   8: repro_num_integer_src_lib_rs_552_22_cc4487::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
