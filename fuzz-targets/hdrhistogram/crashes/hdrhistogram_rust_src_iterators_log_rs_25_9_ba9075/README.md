# hdrhistogram_rust/src/iterators/log.rs:25:9

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: panic
- location: `hdrhistogram_rust/src/iterators/log.rs:25:9`
- last API call: `hdrhistogram::Histogram<u8>::iter_log`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
value_units_per_bucket must be > 0
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
    let histogram = <hdrhistogram::Histogram<u8>>::new(1u8).unwrap();
    let _ = histogram.iter_log(0u64, 0.0f64);
}
```

Run with `cargo run` in this directory (package `repro_hdrhistogram_rust_src_iterators_log_rs_25_9_ba9075`).

## Output

```
thread 'main' (1300330) panicked at /home/rafal/.cargo/git/checkouts/hdrhistogram_rust-8b189d86179db6ba/ea926c4/src/iterators/log.rs:25:9:
value_units_per_bucket must be > 0
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/std/src/panicking.rs:679:5
   1: core::panicking::panic_fmt
             at /rustc/b940084d7eb6a299eb4bfeb8e34901bc051e7ac4/library/core/src/panicking.rs:80:14
   2: <hdrhistogram::iterators::log::Iter<u8>>::new
             at /home/rafal/.cargo/git/checkouts/hdrhistogram_rust-8b189d86179db6ba/ea926c4/src/iterators/log.rs:25:9
   3: <hdrhistogram::Histogram<u8>>::iter_log
             at /home/rafal/.cargo/git/checkouts/hdrhistogram_rust-8b189d86179db6ba/ea926c4/src/lib.rs:1154:9
   4: repro_hdrhistogram_rust_src_iterators_log_rs_25_9_ba9075::main
             at ./src/main.rs:3:23
   5: <fn() as core::ops::function::FnOnce<()>>::call_once
             at /home/rafal/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```
