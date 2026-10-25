# moxcms/src/nd_array.rs:841:49

- crate: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f
- kind: panic
- location: `moxcms/src/nd_array.rs:841:49`
- last API call: `moxcms::Cube::prism_vec3`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
attempt to multiply with overflow
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { git = "https://github.com/awxkee/moxcms", rev = "a61db8babddcb47f8cb46314f2c6577756d7496f", features = ["any_to_any", "avx", "avx512", "avx512_shaper_fixed_point_paths", "avx512_shaper_optimized_paths", "avx_luts", "avx_shaper_fixed_point_paths", "avx_shaper_optimized_paths", "avx_shaper_paths", "extended_range", "in_place", "lut", "neon", "neon_luts", "neon_shaper_fixed_point_paths", "neon_shaper_optimized_paths", "neon_shaper_paths", "options", "sse", "sse_luts", "sse_shaper_fixed_point_paths", "sse_shaper_optimized_paths", "sse_shaper_paths", "sve"] }
```

`src/main.rs`:

```rust
use moxcms::Cube;

fn main() {
    let vec = vec![0.0f32];
    let cube = Cube::new(&vec, 65536usize, 0usize).unwrap();
    let _ = cube.prism_vec3(0.0f32, 0.0f32, 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_moxcms_src_nd_array_rs_841_49_590ddf`).

## Output

```
thread 'main' (356700) panicked at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:841:49:
attempt to multiply with overflow
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_const::panic_const_mul_overflow
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:175:17
   3: <moxcms::nd_array::ArrayFetchVector3f as moxcms::nd_array::ArrayFetch<moxcms::matrix::Vector3<f32>>>::fetch
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:841:49
   4: <moxcms::nd_array::Cube>::prism::<moxcms::matrix::Vector3<f32>, moxcms::nd_array::ArrayFetchVector3f>
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:1192:24
   5: <moxcms::nd_array::Cube>::prism_vec3
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:1249:14
   6: repro_moxcms_src_nd_array_rs_841_49_590ddf::main
             at ./src/main.rs:6:18
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
