# hdrhistogram_rust/src/iterators/quantile.rs:19:9

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: panic
- location: `hdrhistogram_rust/src/iterators/quantile.rs:19:9`
- last API call: `hdrhistogram::Histogram<u64>::iter_quantiles`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
Ticks per half distance must be > 0
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
hdrhistogram = { git = "https://github.com/HdrHistogram/HdrHistogram_rust", rev = "ea926c4c667d7aeb6c1be7bdac3cb68a225730fb" }
```

`src/main.rs`:

```rust
fn main() {
    let histogram = <hdrhistogram::Histogram<u64>>::new(0u8).unwrap();
    let _ = histogram.iter_quantiles(0u32);
}
```

Run with `cargo run` in this directory (package `repro_hdrhistogram_rust_src_iterators_quantile_rs_19_9_53319a`).

## Output

```
thread 'main' (1300137) panicked at /home/rafal/.cargo/git/checkouts/hdrhistogram_rust-8b189d86179db6ba/ea926c4/src/iterators/quantile.rs:19:9:
Ticks per half distance must be > 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <hdrhistogram::iterators::quantile::Iter<u64>>::new
             at /home/rafal/.cargo/git/checkouts/hdrhistogram_rust-8b189d86179db6ba/ea926c4/src/iterators/quantile.rs:19:9
   3: <hdrhistogram::Histogram<u64>>::iter_quantiles
             at /home/rafal/.cargo/git/checkouts/hdrhistogram_rust-8b189d86179db6ba/ea926c4/src/lib.rs:1050:9
   4: repro_hdrhistogram_rust_src_iterators_quantile_rs_19_9_53319a::main
             at ./src/main.rs:3:23
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
