# libm/src/math/support/hex_float.rs:21:39

- crate: https://github.com/rust-lang/compiler-builtins @ f521807de24520770991263387ba8c06e462edf8
- kind: panic
- location: `libm/src/math/support/hex_float.rs:21:39`
- last API call: `libm::support::hf32`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
no hex indicator
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
    let _ = libm::support::hf32("");
}
```

Run with `cargo run` in this directory (package `repro_libm_src_math_support_hex_float_rs_21_39_31542c`).

## Output

```
thread 'main' (1332009) panicked at /home/rafal/.cargo/git/checkouts/compiler-builtins-ffd930bbeb683f76/f521807/libm/src/math/support/hex_float.rs:21:39:
no hex indicator
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display::<&str>
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/panicking.rs:259:5
   3: libm::math::support::hex_float::hf32
             at /home/rafal/.cargo/git/checkouts/compiler-builtins-ffd930bbeb683f76/f521807/libm/src/math/support/hex_float.rs:21:39
   4: repro_libm_src_math_support_hex_float_rs_21_39_31542c::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
