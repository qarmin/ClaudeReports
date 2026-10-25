# vsimd/src/simd256.rs:508:13

- crate: https://github.com/Nugine/simd @ dff9585c080896a9ebdb8d6dcabce17bd69a0874
- kind: panic
- location: `vsimd/src/simd256.rs:508:13`
- last API call: `vsimd::isa::SSSE3::u8x32_swizzle`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
not implemented
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vsimd = { git = "https://github.com/Nugine/simd", rev = "dff9585c080896a9ebdb8d6dcabce17bd69a0874", features = ["alloc", "detect", "std"] }
```

`src/main.rs`:

```rust
use vsimd::hex::ENCODE_LOWER_LUT;

fn main() {
    let s_s_s_e3 = vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap();
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD256>::u8x32_swizzle(s_s_s_e3, ENCODE_LOWER_LUT, ENCODE_LOWER_LUT);
}
```

Run with `cargo run` in this directory (package `repro_vsimd_src_simd256_rs_508_13_448696`).

## Output

```
thread 'main' (1250942) panicked at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd256.rs:508:13:
not implemented
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: <vsimd::isa::SSSE3 as vsimd::simd256::SIMD256>::u8x32_swizzle
             at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd256.rs:508:13
   4: repro_vsimd_src_simd256_rs_508_13_448696::main
             at ./src/main.rs:5:13
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
