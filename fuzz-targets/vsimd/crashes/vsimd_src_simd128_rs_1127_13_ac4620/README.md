# vsimd/src/simd128.rs:1127:13

- crate: https://github.com/Nugine/simd @ dff9585c080896a9ebdb8d6dcabce17bd69a0874
- kind: panic
- location: `vsimd/src/simd128.rs:1127:13`
- last API call: `vsimd::isa::SSE2::i16x8_maddubs`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
internal error: entered unreachable code
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vsimd = { git = "https://github.com/Nugine/simd", rev = "dff9585c080896a9ebdb8d6dcabce17bd69a0874" }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD128>::i16x8_maddubs(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), <vsimd::isa::SSE2 as vsimd::SIMD128>::i64x2_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i64), <vsimd::isa::SSE2 as vsimd::SIMD128>::i8x16_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i8));
}
```

Run with `cargo run` in this directory (package `repro_vsimd_src_simd128_rs_1127_13_ac4620`).

## Output

```
thread 'main' (1251318) panicked at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd128.rs:1127:13:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <vsimd::isa::SSE2 as vsimd::simd128::SIMD128>::i16x8_maddubs
             at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd128.rs:1127:13
   4: repro_vsimd_src_simd128_rs_1127_13_ac4620::main
             at ./src/main.rs:2:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
