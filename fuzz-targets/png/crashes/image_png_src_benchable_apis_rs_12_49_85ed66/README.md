# image-png/src/benchable_apis.rs:12:49

- crate: https://github.com/image-rs/image-png @ b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc
- kind: panic
- location: `image-png/src/benchable_apis.rs:12:49`
- last API call: `png::benchable_apis::unfilter`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
called `Option::unwrap()` on a `None` value
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
png = { git = "https://github.com/image-rs/image-png", rev = "b2f10c6a7e3127d4aaf231f3eae731a33e6eb0bc", features = ["benchmarks", "zlib-rs"] }
```

`src/main.rs`:

```rust
use png::Filter;

fn main() {
    png::benchable_apis::unfilter(Filter::MinEntropy, 0u8, &vec![], &mut vec![]);
}
```

Run with `cargo run` in this directory (package `repro_image_png_src_benchable_apis_rs_12_49_85ed66`).

## Output

```
thread 'main' (1263491) panicked at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/benchable_apis.rs:12:49:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/option.rs:2248:5
   4: <core::option::Option<png::filter::RowFilter>>::unwrap
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1014:21
   5: png::benchable_apis::unfilter
             at /home/rafal/.cargo/git/checkouts/image-png-1d85973eec6d3c42/b2f10c6/src/benchable_apis.rs:12:49
   6: repro_image_png_src_benchable_apis_rs_12_49_85ed66::main
             at ./src/main.rs:4:5
   7: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
