# timeout in jxl_coding::read_clusters

- crate: https://github.com/tirr-c/jxl-oxide @ c0cc4c7ea57c1207f38ff2970d94757470613be4
- kind: timeout
- last API call: `jxl_coding::read_clusters`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (skipped while revalidating)

## Message

```
process ended abnormally (timeout) while calling jxl_coding::read_clusters
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
jxl-oxide = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-coding = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
jxl-bitstream = { git = "https://github.com/tirr-c/jxl-oxide", rev = "c0cc4c7ea57c1207f38ff2970d94757470613be4" }
```

`src/main.rs`:

```rust
use jxl_bitstream::Bitstream;

fn main() {
    let vec = vec![73u8, 73u8, 42u8, 0u8, 39u8, 0u8];
    let mut bitstream = Bitstream::new(&vec);
    let _ = jxl_coding::read_clusters(&mut bitstream, u32::MAX);
}
```

Run with `cargo run` in this directory (package `repro_timeout_in_jxl_coding_read_clusters_fe6752`).

