# timeout in hdrhistogram::SyncHistogram<u32>::from<hdrhistogram::Histogram<u32>>

- crate: https://github.com/HdrHistogram/HdrHistogram_rust @ ea926c4c667d7aeb6c1be7bdac3cb68a225730fb
- kind: timeout
- last API call: `hdrhistogram::SyncHistogram<u32>::from<hdrhistogram::Histogram<u32>>`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling hdrhistogram::SyncHistogram<u32>::from<hdrhistogram::Histogram<u32>>
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
    let mut histogram = <hdrhistogram::Histogram<u32> as ::core::convert::From<hdrhistogram::Histogram<u32>>>::from(<hdrhistogram::Histogram<u32>>::new(1u8).unwrap());
    let _ = histogram.record(u64::MAX);
    let _ = <hdrhistogram::SyncHistogram<u32> as ::core::convert::From<hdrhistogram::Histogram<u32>>>::from(histogram.clone_correct(2u64));
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_hdrhistogram_synchistogram_u32_from_hdrhistogram__b96ea0`).

