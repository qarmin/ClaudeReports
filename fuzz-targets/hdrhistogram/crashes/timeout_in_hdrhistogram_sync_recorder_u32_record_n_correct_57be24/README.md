# timeout in hdrhistogram::sync::Recorder<u32>::record_n_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::sync::Recorder<u32>::record_n_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::sync::Recorder<u32>::record_n_correct
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
use hdrhistogram::serialization::DeserializeError;

fn main() {
    let _ = AdditionError::OtherAddendValueExceedsRange;
    let histogram = <hdrhistogram::Histogram<u32>>::new(0u8).unwrap();
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_n_correct(u64::MAX, 1585395343u32, 1u64);
    let _ = DeserializeError::EncodedArrayTooLong;
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_sync_recorder_u32_record_n_correct_57be24`).

