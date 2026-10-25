# num-integer/src/lib.rs:845:1

- crate: https://github.com/rust-num/num-integer @ 765fd9b3eb9397471d41b9f613e530f6d9d6383f
- kind: panic
- location: `num-integer/src/lib.rs:845:1`
- last API call: `num_integer::div_mod_floor[i32]`
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
num-integer = { git = "https://github.com/rust-num/num-integer", rev = "765fd9b3eb9397471d41b9f613e530f6d9d6383f" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = num_integer::div_mod_floor::<i32>(0i32, 0i32);
}
```

Run with `cargo run` in this directory (package `repro_num_integer_src_lib_rs_845_1_ca3d69`).

## Output

```
thread 'main' (388090) panicked at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:845:1:
attempt to divide by zero
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i32 as num_integer::Integer>::div_rem
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:592:18
   4: <i32 as num_integer::Integer>::div_mod_floor
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:472:35
   5: num_integer::div_mod_floor::<i32>
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/lib.rs:411:7
   6: repro_num_integer_src_lib_rs_845_1_ca3d69::main
             at ./src/main.rs:2:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
