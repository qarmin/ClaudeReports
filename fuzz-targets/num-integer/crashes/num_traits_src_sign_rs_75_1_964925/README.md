# num-traits/src/sign.rs:75:1

- crate: https://github.com/rust-num/num-integer @ 765fd9b3eb9397471d41b9f613e530f6d9d6383f
- kind: panic
- location: `num-traits/src/sign.rs:75:1`
- source: https://docs.rs/crate/num-traits/0.2.19/source/src/sign.rs#75
- last API call: `num_integer::lcm[i16]`
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
    let _ = num_integer::lcm::<i16>(i16::MIN, i16::MIN);
}
```

Run with `cargo run` in this directory (package `repro_num_traits_src_sign_rs_75_1_964925`).

## Output

```
thread 'main' (388752) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/num-traits-0.2.19/src/sign.rs:75:1:
attempt to negate with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_neg_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i16 as num_traits::sign::Signed>::abs
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/num-traits-0.2.19/src/sign.rs:49:41
   4: <i16 as num_integer::Integer>::gcd
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:513:41
   5: <i16 as num_integer::Integer>::gcd_lcm
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:562:32
   6: <i16 as num_integer::Integer>::lcm
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:552:22
   7: num_integer::lcm::<i16>
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:428:7
   8: repro_num_traits_src_sign_rs_75_1_964925::main
             at ./src/main.rs:2:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
