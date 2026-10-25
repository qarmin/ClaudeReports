# vsimd/src/simd128.rs:660:13

- crate: https://github.com/Nugine/simd @ dff9585c080896a9ebdb8d6dcabce17bd69a0874
- kind: panic
- location: `vsimd/src/simd128.rs:660:13`
- last API call: `vsimd::isa::SSE2::u16x16_bswap`
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
vsimd = { git = "https://github.com/Nugine/simd", rev = "dff9585c080896a9ebdb8d6dcabce17bd69a0874", features = ["alloc", "detect", "std"] }
```

`src/main.rs`:

```rust
fn main() {
    let _ = <vsimd::isa::SSE2 as vsimd::SIMD256>::u16x16_bswap(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), <vsimd::isa::SSE41 as vsimd::SIMD256>::u64x4_splat(vsimd::isa::detect::<vsimd::isa::SSE41>().unwrap(), 0u64));
}
```

Run with `cargo run` in this directory (package `repro_vsimd_src_simd128_rs_660_13_b4f364`).

## Output

```
thread 'main' (1250199) panicked at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd128.rs:660:13:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <vsimd::isa::SSE2 as vsimd::simd128::SIMD128>::u16x8_bswap
             at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd128.rs:660:13
   4: <vsimd::isa::SSE2 as vsimd::simd256::SIMD256>::u16x16_bswap
             at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd256.rs:35:18
   5: repro_vsimd_src_simd128_rs_660_13_b4f364::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
