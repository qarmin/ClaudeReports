# timeout in hdrhistogram::sync::Recorder<u16>::record_n_correct

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::sync::Recorder<u16>::record_n_correct`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::sync::Recorder<u16>::record_n_correct
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
    let sync_histogram = <hdrhistogram::SyncHistogram<u16> as ::core::convert::From<hdrhistogram::Histogram<u16>>>::from(<hdrhistogram::Histogram<u16>>::new(0u8).unwrap());
    let mut recorder = sync_histogram.recorder();
    let _ = recorder.record_n_correct(10000000000000000000u64, 82u16, 1000u64);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_sync_recorder_u16_record_n_correct_c46006`).

