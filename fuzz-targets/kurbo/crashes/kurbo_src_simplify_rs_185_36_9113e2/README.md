# kurbo/src/simplify.rs:185:36

- crate: https://github.com/linebender/kurbo @ 9366cda80ebb56f41e5d09509a89dbfe81bc4f5b
- kind: panic
- location: `kurbo/src/simplify.rs:185:36`
- last API call: `kurbo::simplify::SimplifyBezPath::moment_integrals`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
index out of bounds: the len is 0 but the index is 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
kurbo = { git = "https://github.com/linebender/kurbo", rev = "9366cda80ebb56f41e5d09509a89dbfe81bc4f5b" }
```

`src/main.rs`:

```rust
use kurbo::simplify::SimplifyBezPath;

fn main() {
    let _ = <kurbo::simplify::SimplifyBezPath as kurbo::ParamCurveFit>::moment_integrals(&SimplifyBezPath::new(vec![]), f64::NAN..0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_kurbo_src_simplify_rs_185_36_9113e2`).

## Output

```
thread 'main' (411908) panicked at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:185:36:
index out of bounds: the len is 0 but the index is 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_bounds_check
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:271:5
   3: <usize as core::slice::index::SliceIndex<[kurbo::simplify::SimplifyCubic]>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:238:10
   4: <[kurbo::simplify::SimplifyCubic] as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:19:15
   5: <alloc::vec::Vec<kurbo::simplify::SimplifyCubic> as core::ops::index::Index<usize>>::index
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3934:9
   6: <kurbo::simplify::SimplifyBezPath>::moment_integrals
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:185:36
   7: <kurbo::simplify::SimplifyBezPath as kurbo::fit::ParamCurveFit>::moment_integrals
             at /home/rafal/.cargo/git/checkouts/kurbo-29ecae83f8765c40/9366cda/kurbo/src/simplify.rs:221:18
   8: repro_kurbo_src_simplify_rs_185_36_9113e2::main
             at ./src/main.rs:4:13
   9: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
