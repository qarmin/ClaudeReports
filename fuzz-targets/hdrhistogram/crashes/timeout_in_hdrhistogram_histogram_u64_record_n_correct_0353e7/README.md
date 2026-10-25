# timeout in hdrhistogram::Histogram<u64>::record_n_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::Histogram<u64>::record_n_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::Histogram<u64>::record_n_correct
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
    let mut histogram = <hdrhistogram::Histogram<u64>>::new_with_max(1000000000000000000u64, 0u8).unwrap();
    let _ = histogram.record_n_correct(4294967296u64, 1u64, 1u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_histogram_u64_record_n_correct_0353e7`).

