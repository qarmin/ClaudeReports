# libm/src/math/support/modular.rs:22:5

- crate: https://github.com/rust-lang/compiler-builtins @ f521807de24520770991263387ba8c06e462edf8
- kind: panic
- location: `libm/src/math/support/modular.rs:22:5`
- last API call: `libm::support::linear_mul_reduction[u8]`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
assertion failed: y <= U::MAX >> 2
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
    let _ = libm::support::linear_mul_reduction::<u8>(0u8, 0u32, 100u8);
}
```

Run with `cargo run` in this directory (package `repro_libm_src_math_support_modular_rs_22_5_8c4a3a`).

## Output

```
thread 'main' (461792) panicked at /home/rafal/.cargo/git/checkouts/compiler-builtins-ffd930bbeb683f76/f521807/libm/src/math/support/modular.rs:22:5:
assertion failed: y <= U::MAX >> 2
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: libm::math::support::modular::linear_mul_reduction::<u8>
             at /home/rafal/.cargo/git/checkouts/compiler-builtins-ffd930bbeb683f76/f521807/libm/src/math/support/modular.rs:22:5
   4: repro_libm_src_math_support_modular_rs_22_5_8c4a3a::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
