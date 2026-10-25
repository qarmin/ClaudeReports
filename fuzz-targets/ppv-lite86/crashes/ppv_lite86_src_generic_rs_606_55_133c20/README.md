# ppv-lite86/src/generic.rs:606:55

- crate: https://github.com/cryptocorrosion/cryptocorrosion @ aadd0158fc0000105404925fb2730eac11db1125
- kind: panic
- location: `ppv-lite86/src/generic.rs:606:55`
- last API call: `ppv_lite86::generic::GenericMachine::read_be[u64x2_generic]`
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
    let generic_machine = GenericMachine;
    let _ = <ppv_lite86::generic::GenericMachine as ppv_lite86::Machine>::read_be::<ppv_lite86::generic::u64x2_generic>(generic_machine, &vec![]);
}
```

Run with `cargo run` in this directory (package `repro_ppv_lite86_src_generic_rs_606_55_133c20`).

## Output

```
thread 'main' (447758) panicked at /home/rafal/.cargo/git/checkouts/cryptocorrosion-0af2014cc42b54fe/aadd015/utils-simd/ppv-lite86/src/generic.rs:606:55:
called `Result::unwrap()` on an `Err` value: SizeError
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::result::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/result.rs:1871:5
   3: <core::result::Result<ppv_lite86::generic::u64x2_generic, zerocopy::error::SizeError<&[u8], ppv_lite86::generic::u64x2_generic>>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:1232:23
   4: <ppv_lite86::generic::u64x2_generic as ppv_lite86::types::StoreBytes>::unsafe_read_be
             at /home/rafal/.cargo/git/checkouts/cryptocorrosion-0af2014cc42b54fe/aadd015/utils-simd/ppv-lite86/src/generic.rs:606:55
   5: <ppv_lite86::generic::GenericMachine as ppv_lite86::types::Machine>::read_be::<ppv_lite86::generic::u64x2_generic>
             at /home/rafal/.cargo/git/checkouts/cryptocorrosion-0af2014cc42b54fe/aadd015/utils-simd/ppv-lite86/src/types.rs:271:18
   6: repro_ppv_lite86_src_generic_rs_606_55_133c20::main
             at ./src/main.rs:5:13
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
