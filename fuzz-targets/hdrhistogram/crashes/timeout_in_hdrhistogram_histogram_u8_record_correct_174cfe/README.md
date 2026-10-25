# timeout in hdrhistogram::Histogram<u8>::record_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::Histogram<u8>::record_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::Histogram<u8>::record_correct
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
hdrhistogram = { git = "https://github.com/HdrHistogram/HdrHistogram_rust", rev = "ea926c4c667d7aeb6c1be7bdac3cb68a225730fb" }
```

`src/main.rs`:

```rust
use hdrhistogram::errors::UsizeTypeTooSmall;

fn main() {
    let _ = UsizeTypeTooSmall;
    let mut histogram = <hdrhistogram::Histogram<u8>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_with_bounds(7u64, 13826826444047866644u64, 1u8).unwrap()));
    let _ = histogram.record_correct(u64::MAX, 4294967296u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_histogram_u8_record_correct_174cfe`).

