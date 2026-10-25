# vsimd/src/unified.rs:907:9

- crate: https://github.com/Nugine/simd @ dff9585c080896a9ebdb8d6dcabce17bd69a0874
- kind: panic
- location: `vsimd/src/unified.rs:907:9`
- last API call: `vsimd::isa::SSSE3::u16x8_min`
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
    let _ = <vsimd::isa::SSSE3 as vsimd::SIMD128>::u16x8_min(vsimd::isa::detect::<vsimd::isa::SSSE3>().unwrap(), <vsimd::isa::SSE2 as vsimd::SIMD128>::i64x2_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i64), <vsimd::isa::SSE2 as vsimd::SIMD128>::i16x8_splat(vsimd::isa::detect::<vsimd::isa::SSE2>().unwrap(), 0i16));
}
```

Run with `cargo run` in this directory (package `repro_vsimd_src_unified_rs_907_9_8c0dc2`).

## Output

```
thread 'main' (1250639) panicked at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/unified.rs:907:9:
internal error: entered unreachable code
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: vsimd::unified::min::<vsimd::isa::SSSE3, u16, vsimd::vector::V128>
             at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/unified.rs:907:9
   4: <vsimd::isa::SSSE3 as vsimd::simd128::SIMD128>::u16x8_min
             at /home/rafal/.cargo/git/checkouts/simd-55ad80aa8858c82d/dff9585/crates/vsimd/src/simd128.rs:586:9
   5: repro_vsimd_src_unified_rs_907_9_8c0dc2::main
             at ./src/main.rs:2:13
   6: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
