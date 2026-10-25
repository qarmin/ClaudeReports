# memchr/src/arch/all/twoway.rs:341:17

- crate: https://github.com/BurntSushi/memchr @ bd6068c30e9074a90c285e47912fa0b047d07597
- kind: panic
- location: `memchr/src/arch/all/twoway.rs:341:17`
- last API call: `memchr::arch::all::twoway::FinderRev::rfind`
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
memchr = { git = "https://github.com/BurntSushi/memchr", rev = "bd6068c30e9074a90c285e47912fa0b047d07597" }
```

`src/main.rs`:

```rust
use memchr::arch::all::twoway::FinderRev;

fn main() {
    let finder_rev = FinderRev::new(&vec![0u8, 0u8]);
    let _ = finder_rev.rfind(&vec![0u8], &vec![109u8]);
}
```

Run with `cargo run` in this directory (package `repro_memchr_src_arch_all_twoway_rs_341_17_749707`).

## Output

```
thread 'main' (448605) panicked at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/all/twoway.rs:341:17:
attempt to subtract with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_sub_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <memchr::arch::all::twoway::FinderRev>::rfind_small_imp
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/all/twoway.rs:341:17
   4: <memchr::arch::all::twoway::FinderRev>::rfind
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/all/twoway.rs:308:22
   5: repro_memchr_src_arch_all_twoway_rs_341_17_749707::main
             at ./src/main.rs:5:24
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
