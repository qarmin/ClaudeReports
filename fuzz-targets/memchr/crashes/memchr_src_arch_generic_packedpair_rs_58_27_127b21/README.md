# memchr/src/arch/generic/packedpair.rs:58:27

- crate: https://github.com/BurntSushi/memchr @ bd6068c30e9074a90c285e47912fa0b047d07597
- kind: panic
- location: `memchr/src/arch/generic/packedpair.rs:58:27`
- last API call: `memchr::arch::x86_64::sse2::packedpair::Finder::with_pair`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 1
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
memchr = { git = "https://github.com/BurntSushi/memchr", rev = "bd6068c30e9074a90c285e47912fa0b047d07597" }
```

`src/main.rs`:

```rust
use memchr::arch::all::packedpair::Pair;
use memchr::arch::x86_64::sse2::packedpair::Finder;

fn main() {
    let _ = Finder::with_pair(&vec![], Pair::new(&vec![110u8, 79u8]).unwrap());
}
```

Run with `cargo run` in this directory (package `repro_memchr_src_arch_generic_packedpair_rs_58_27_127b21`).

## Output

```
thread 'main' (448479) panicked at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/generic/packedpair.rs:58:27:
index out of bounds: the len is 0 but the index is 1
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <memchr::arch::generic::packedpair::Finder<core::core_arch::x86::__m128i>>::new
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/generic/packedpair.rs:58:27
   4: <memchr::arch::x86_64::sse2::packedpair::Finder>::with_pair_impl
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/x86_64/sse2/packedpair.rs:66:22
   5: <memchr::arch::x86_64::sse2::packedpair::Finder>::with_pair
             at /home/rafal/.cargo/git/checkouts/memchr-778e03f2cc4efb42/bd6068c/src/arch/x86_64/sse2/packedpair.rs:51:27
   6: repro_memchr_src_arch_generic_packedpair_rs_58_27_127b21::main
             at ./src/main.rs:5:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
