# timeout in hdrhistogram::sync::Recorder<u8>::record_n_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::sync::Recorder<u8>::record_n_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::sync::Recorder<u8>::record_n_correct
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
    let histogram = <hdrhistogram::Histogram<u8>>::new_from::<u16>(&<hdrhistogram::Histogram<u16>>::new_from::<u32>(&<hdrhistogram::Histogram<u32>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_with_max(u64::MAX, 2u8).unwrap())));
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_n_correct(4380101545244474084u64, 26u8, 7u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_sync_recorder_u8_record_n_correct_e1bdaf`).

