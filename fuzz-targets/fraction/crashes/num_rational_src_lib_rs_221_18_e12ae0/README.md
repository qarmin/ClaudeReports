# num-rational/src/lib.rs:221:18

- crate: https://github.com/dnsl48/fraction @ 53d04cddb4640d1fe7756c289b3ce34be14da504
- kind: panic
- location: `num-rational/src/lib.rs:221:18`
- source: https://docs.rs/crate/num-rational/0.4.2/source/src/lib.rs#221
- last API call: `fraction::prelude::GenericFraction<u16>::ceil`
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
fraction = { git = "https://github.com/dnsl48/fraction", rev = "53d04cddb4640d1fe7756c289b3ce34be14da504" }
```

`src/main.rs`:

```rust
fn main() {
    let generic_fraction = <fraction::prelude::GenericFraction<u16>>::new_neg::<u16, u16>(u16::MAX, 1u16);
    let generic_fraction2 = generic_fraction.abs();
    let _ = generic_fraction2.ceil();
}
```

Run with `cargo run` in this directory (package `repro_num_rational_src_lib_rs_221_18_e12ae0`).

## Output

```
thread 'main' (415418) panicked at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/num-rational-0.4.2/src/lib.rs:221:18:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <u16 as core::ops::arith::Add>::add
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:104:45
   4: <num_rational::Ratio<u16>>::ceil
             at /home/rafal/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/num-rational-0.4.2/src/lib.rs:221:18
   5: <fraction::fraction::generic_fraction::GenericFraction<u16>>::ceil
             at /home/rafal/.cargo/git/checkouts/fraction-22db6e5452e03bd8/53d04cd/src/fraction/generic_fraction.rs:1027:57
   6: repro_num_rational_src_lib_rs_221_18_e12ae0::main
             at ./src/main.rs:4:31
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
