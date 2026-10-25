# timeout in hdrhistogram::Histogram<u16>::record_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::Histogram<u16>::record_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::Histogram<u16>::record_correct
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
hdrhistogram = { git = "https://github.com/HdrHistogram/HdrHistogram_rust", rev = "ea926c4c667d7aeb6c1be7bdac3cb68a225730fb" }
```

`src/main.rs`:

```rust
use hdrhistogram::serialization::V2DeflateSerializer;
use hdrhistogram::serialization::interval_log::IntervalLogWriterBuilder;

fn main() {
    let histogram = <hdrhistogram::Histogram<u64>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_from::<u16>(&<hdrhistogram::Histogram<u16>>::new(0u8).unwrap()));
    let _ = IntervalLogWriterBuilder::default();
    let _ = V2DeflateSerializer::default();
    let mut histogram2 = <hdrhistogram::Histogram<u16>>::new_from::<u64>(&histogram);
    let _ = histogram2.record_correct(100000000000000000u64, 1u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_histogram_u16_record_correct_ed1af2`).

