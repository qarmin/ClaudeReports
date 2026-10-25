# num-integer/src/roots.rs:164:1

- crate: https://github.com/rust-num/num-integer @ 765fd9b3eb9397471d41b9f613e530f6d9d6383f
- kind: panic
- location: `num-integer/src/roots.rs:164:1`
- last API call: `num_integer::nth_root[i16]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
even roots of a negative are imaginary
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
    let _ = num_integer::nth_root::<i16>(-1i16, 0u32);
}
```

Run with `cargo run` in this directory (package `repro_num_integer_src_roots_rs_164_1_d5228d`).

## Output

```
thread 'main' (388669) panicked at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/roots.rs:164:1:
even roots of a negative are imaginary
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <i16 as num_integer::roots::Roots>::nth_root
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/roots.rs:140:21
   4: num_integer::roots::nth_root::<i16>
             at /home/rafal/.cargo/git/checkouts/num-integer-b0beb1a944753abc/765fd9b/src/roots.rs:129:7
   5: repro_num_integer_src_roots_rs_164_1_d5228d::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
