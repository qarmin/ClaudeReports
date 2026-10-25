# memchr/src/arch/all/twoway.rs:260:24

- crate: https://github.com/BurntSushi/memchr @ bd6068c30e9074a90c285e47912fa0b047d07597
- kind: panic
- location: `memchr/src/arch/all/twoway.rs:260:24`
- last API call: `memchr::arch::all::twoway::Finder::find`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 1 but the index is 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
memchr = { git = "https://github.com/BurntSushi/memchr", rev = "bd6068c30e9074a90c285e47912fa0b047d07597" }
```

`src/main.rs`:

```rust
use memchr::arch::all::twoway::Finder;

fn main() {
    let finder = Finder::new(&vec![164u8, 164u8, 0u8]);
    let _ = finder.find(&vec![0u8], &vec![0u8]);
}
```

Run with `cargo run` in this directory (package `repro_memchr_src_arch_all_twoway_rs_260_24_dcad97`).

## Output

```
thread 'main' (448357) panicked at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/all/twoway.rs:260:24:
index out of bounds: the len is 1 but the index is 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <memchr::arch::all::twoway::Finder>::find_large_imp
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/all/twoway.rs:260:24
   4: <memchr::arch::all::twoway::Finder>::find_with_prefilter
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/all/twoway.rs:162:22
   5: <memchr::arch::all::twoway::Finder>::find
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/all/twoway.rs:138:14
   6: repro_memchr_src_arch_all_twoway_rs_260_24_dcad97::main
             at ./src/main.rs:5:20
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
