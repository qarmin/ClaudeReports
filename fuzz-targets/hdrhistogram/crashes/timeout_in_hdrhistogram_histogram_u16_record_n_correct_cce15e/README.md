# timeout in hdrhistogram::Histogram<u16>::record_n_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::Histogram<u16>::record_n_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::Histogram<u16>::record_n_correct
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
hdrhistogram = { git = "https://github.com/HdrHistogram/HdrHistogram_rust", rev = "ea926c4c667d7aeb6c1be7bdac3cb68a225730fb" }
```

`src/main.rs`:

```rust
use hdrhistogram::errors::AdditionError;
use hdrhistogram::serialization::V2DeflateSerializeError;
use hdrhistogram::serialization::V2SerializeError;
use hdrhistogram::serialization::interval_log::Tag;

fn main() {
    let string = "(\\)".to_string();
    let tag = Tag::new(&string).unwrap();
    let _ = tag.as_str();
    let mut histogram = <hdrhistogram::Histogram<u16>>::new_from::<u8>(&<hdrhistogram::Histogram<u8>>::new_from::<u32>(&<hdrhistogram::Histogram<u32> as ::core::convert::From<hdrhistogram::Histogram<u32>>>::from(<hdrhistogram::Histogram<u32>>::new(0u8).unwrap())).clone_correct(256u64));
    let _ = histogram.record_n_correct(u64::MAX, 56596u16, 82u64);
    let _ = V2DeflateSerializeError::InternalSerializationError(V2SerializeError::UsizeTypeTooSmall);
    let _ = AdditionError::ResizeFailedUsizeTypeTooSmall;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_histogram_u16_record_n_correct_cce15e`).

