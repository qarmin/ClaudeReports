# rand_distr/src/hypergeometric.rs:204:53

- crate: https://github.com/rust-random/rand_distr @ d65b9bbf991e56d8a097a35d934e6f93d9194ac0
- kind: panic
- location: `rand_distr/src/hypergeometric.rs:204:53`
- last API call: `rand_distr::Hypergeometric::new`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to add with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
rand_distr = { git = "https://github.com/rust-random/rand_distr", rev = "d65b9bbf991e56d8a097a35d934e6f93d9194ac0" }
```

`src/main.rs`:

```rust
use rand_distr::Hypergeometric;

fn main() {
    let _ = Hypergeometric::new(u64::MAX, 0u64, 0u64);
}
```

Run with `cargo run` in this directory (package `repro_rand_distr_src_hypergeometric_rs_204_53_540aeb`).

## Output

```
thread 'main' (577495) panicked at /home/rafal/.cargo/git/checkouts/rand_distr-ac891c4404ad706f/d65b9bb/src/hypergeometric.rs:204:53:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <rand_distr::hypergeometric::Hypergeometric>::new
             at /home/rafal/.cargo/git/checkouts/rand_distr-ac891c4404ad706f/d65b9bb/src/hypergeometric.rs:204:53
   4: repro_rand_distr_src_hypergeometric_rs_204_53_540aeb::main
             at ./src/main.rs:4:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
