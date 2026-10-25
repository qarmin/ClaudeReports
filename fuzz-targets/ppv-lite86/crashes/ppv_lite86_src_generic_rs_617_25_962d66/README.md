# ppv-lite86/src/generic.rs:617:25

- crate: https://github.com/cryptocorrosion/cryptocorrosion @ aadd0158fc0000105404925fb2730eac11db1125
- kind: panic
- location: `ppv-lite86/src/generic.rs:617:25`
- last API call: `ppv_lite86::generic::u64x2_generic::write_be`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Result::unwrap()` on an `Err` value: SizeError
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
ppv-lite86 = { git = "https://github.com/cryptocorrosion/cryptocorrosion", rev = "aadd0158fc0000105404925fb2730eac11db1125", features = ["no_simd", "simd", "std"] }
```

`src/main.rs`:

```rust
use ppv_lite86::generic::GenericMachine;

fn main() {
    let u64x2_generic = <ppv_lite86::generic::u64x2_generic as ppv_lite86::Swap64>::swap4(<ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_be::<ppv_lite86::generic::u64x2_generic>(GenericMachine, &vec![66u8, 77u8, 70u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 54u8, 0u8, 0u8, 0u8, 40u8, 0u8]));
    <ppv_lite86::generic::u64x2_generic as ppv_lite86::StoreBytes>::write_be(u64x2_generic, &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_ppv_lite86_src_generic_rs_617_25_962d66`).

## Output

```
thread 'main' (447900) panicked at /home/rafal/.cargo/git/checkouts/cryptocorrosion-0af2014cc42b54fe/aadd015/utils-simd/ppv-lite86/src/generic.rs:617:25:
called `Result::unwrap()` on an `Err` value: SizeError
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<(), zerocopy::error::SizeError<&ppv_lite86::generic::u64x2_generic, &mut [u8]>>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <ppv_lite86::generic::u64x2_generic as ppv_lite86::types::StoreBytes>::write_be
             at /home/rafal/.cargo/git/checkouts/cryptocorrosion-0af2014cc42b54fe/aadd015/utils-simd/ppv-lite86/src/generic.rs:617:25
   5: repro_ppv_lite86_src_generic_rs_617_25_962d66::main
             at ./src/main.rs:5:5
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
