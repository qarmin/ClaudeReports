# moxcms/src/nd_array.rs:211:28

- crate: https://github.com/awxkee/moxcms @ a61db8babddcb47f8cb46314f2c6577756d7496f
- kind: panic
- location: `moxcms/src/nd_array.rs:211:28`
- last API call: `moxcms::Hypercube::prism_vec4`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
range start index 17179869180 out of range for slice of length 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
moxcms = { git = "https://github.com/awxkee/moxcms", rev = "a61db8babddcb47f8cb46314f2c6577756d7496f", features = ["any_to_any", "avx", "avx512", "avx512_shaper_fixed_point_paths", "avx512_shaper_optimized_paths", "avx_luts", "avx_shaper_fixed_point_paths", "avx_shaper_optimized_paths", "avx_shaper_paths", "extended_range", "in_place", "lut", "neon", "neon_luts", "neon_shaper_fixed_point_paths", "neon_shaper_optimized_paths", "neon_shaper_paths", "options", "sse", "sse_luts", "sse_shaper_fixed_point_paths", "sse_shaper_optimized_paths", "sse_shaper_paths", "sve"] }
```

`src/main.rs`:

```rust
use moxcms::Hypercube;
use moxcms::LutStore;

fn main() {
    let lut_store = LutStore::Store16(vec![]);
    let vec = lut_store.to_clut_f32();
    let hypercube = Hypercube::new(&vec, 0usize, 0usize).unwrap();
    let _ = hypercube.prism_vec4(0.0f32, 0.0f32, 0.0f32, 0.0f32);
}
```

Run with `cargo run` in this directory (package `repro_moxcms_src_nd_array_rs_211_28_e37e69`).

## Output

```
thread 'main' (356781) panicked at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:211:28:
range start index 17179869180 out of range for slice of length 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::slice::index::slice_index_fail
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panic.rs:177:9
   3: <core::ops::range::Range<usize> as core::slice::index::SliceIndex<[f32]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:413:13
   4: <[f32] as core::ops::index::Index<core::ops::range::Range<usize>>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <moxcms::nd_array::Fetch4Vec4 as moxcms::nd_array::Fetcher4<moxcms::matrix::Vector4<f32>>>::fetch
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:211:28
   6: <moxcms::nd_array::Hypercube>::prism::<moxcms::matrix::Vector4<f32>, moxcms::nd_array::Fetch4Vec4>
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:535:20
   7: <moxcms::nd_array::Hypercube>::prism_vec4
             at /home/rafal/.cargo/git/checkouts/moxcms-29ba2364210b7bba/a61db8b/src/nd_array.rs:639:14
   8: repro_moxcms_src_nd_array_rs_211_28_e37e69::main
             at ./src/main.rs:8:23
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
