# num-bigint/src/algorithms/add.rs:16:5

- crate: https://github.com/dignifiedquire/num-bigint @ e53190dc4bdaa71a13c539dbd3898ac1d261a5b2
- kind: panic
- location: `num-bigint/src/algorithms/add.rs:16:5`
- last API call: `num_bigint_dig::algorithms::add2`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: a.len() >= b.len()
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
num-bigint-dig = { git = "https://github.com/dignifiedquire/num-bigint", rev = "e53190dc4bdaa71a13c539dbd3898ac1d261a5b2" }
```

`src/main.rs`:

```rust
fn main() {
    num_bigint_dig::algorithms::add2(&mut vec![], &[0u64]);
}
```

Run with `cargo run` in this directory (package `repro_num_bigint_src_algorithms_add_rs_16_5_df5694`).

## Output

```
thread 'main' (1266439) panicked at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/add.rs:16:5:
assertion failed: a.len() >= b.len()
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: num_bigint_dig::algorithms::add::__add2
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/add.rs:16:5
   4: num_bigint_dig::algorithms::add::add2
             at /home/rafal/.cargo/git/checkouts/num-bigint-c2134c7b1260c879/e53190d/src/algorithms/add.rs:43:17
   5: repro_num_bigint_src_algorithms_add_rs_16_5_df5694::main
             at ./src/main.rs:2:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
