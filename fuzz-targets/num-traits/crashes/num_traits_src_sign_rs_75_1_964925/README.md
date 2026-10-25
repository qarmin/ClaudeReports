# num-traits/src/sign.rs:75:1

- crate: https://github.com/rust-num/num-traits @ 1166177ec398601d6dd4d88891cf7674f3978888
- kind: panic
- location: `num-traits/src/sign.rs:75:1`
- last API call: `num_traits::abs_sub[i32]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to subtract with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-traits = { git = "https://github.com/rust-num/num-traits", rev = "1166177ec398601d6dd4d88891cf7674f3978888" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = num_traits::abs_sub::<i32>(0i32, i32::MIN);
}
```

Run with `cargo run` in this directory (package `repro_num_traits_src_sign_rs_75_1_964925`).

## Output

```
thread 'main' (497338) panicked at /home/rafal/.cargo/git/checkouts/num-traits-b0e77445b190c6c0/1166177/src/sign.rs:75:1:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <i32 as num_traits::sign::Signed>::abs_sub
             at /home/rafal/.cargo/git/checkouts/num-traits-b0e77445b190c6c0/1166177/src/sign.rs:54:49
   4: num_traits::sign::abs_sub::<i32>
             at /home/rafal/.cargo/git/checkouts/num-traits-b0e77445b190c6c0/1166177/src/sign.rs:172:7
   5: repro_num_traits_src_sign_rs_75_1_964925::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
