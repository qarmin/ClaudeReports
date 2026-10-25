# timeout in hdrhistogram::Histogram<u32>::record_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::Histogram<u32>::record_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::Histogram<u32>::record_correct
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
    let mut histogram = <hdrhistogram::Histogram<u32> as ::core::convert::From<hdrhistogram::Histogram<u32>>>::from(<hdrhistogram::Histogram<u32>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new(1u8).unwrap()))));
    let _ = histogram.record_correct(10000000000000000000u64, 2u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_histogram_u32_record_correct_4f8402`).

