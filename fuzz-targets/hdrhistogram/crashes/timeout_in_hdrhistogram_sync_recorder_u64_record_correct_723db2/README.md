# timeout in hdrhistogram::sync::Recorder<u64>::record_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::sync::Recorder<u64>::record_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::sync::Recorder<u64>::record_correct
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
    let histogram = <hdrhistogram::Histogram<u64>>::new_from::<u64>(&<hdrhistogram::Histogram<u64>>::new_from::<u8>(&<hdrhistogram::Histogram<u8>>::new_from::<u16>(&<hdrhistogram::Histogram<u16>>::new_with_max(u64::MAX, 1u8).unwrap())));
    let sync_histogram = histogram.clone().into_sync();
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_correct(18422672277416534739u64, 4294967295u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_sync_recorder_u64_record_correct_723db2`).

