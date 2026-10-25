# libm/src/math/support/modular.rs:42:5

- crate: https://github.com/rust-lang/compiler-builtins @ f521807de24520770991263387ba8c06e462edf8
- kind: panic
- location: `libm/src/math/support/modular.rs:42:5`
- last API call: `libm::support::linear_mul_reduction[u16]`
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
libm = { git = "https://github.com/rust-lang/compiler-builtins", rev = "f521807de24520770991263387ba8c06e462edf8", features = ["arch", "force-soft-floats", "unstable-float", "unstable-public-internals"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = libm::support::linear_mul_reduction::<u16>(0u16, u32::MAX, 1000u16);
}
```

Run with `cargo run` in this directory (package `repro_libm_src_math_support_modular_rs_42_5_21e813`).

## Output

```
thread 'main' (461734) panicked at /home/rafal/.cargo/git/checkouts/compiler-builtins-ffd930bbeb683f76/f521807/libm/src/math/support/modular.rs:42:5:
attempt to add with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_add_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: libm::math::support::modular::linear_mul_reduction::<u16>
             at /home/rafal/.cargo/git/checkouts/compiler-builtins-ffd930bbeb683f76/f521807/libm/src/math/support/modular.rs:42:5
   4: repro_libm_src_math_support_modular_rs_42_5_21e813::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
