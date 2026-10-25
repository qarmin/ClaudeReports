# out of memory in vello_common::paint::ImageSource::OpaqueId

- crate: https://github.com/linebender/vello @ 9cbe466bded9fff6cf7c1dd74246e95fe74f33c0
- kind: out of memory
- last API call: `vello_common::paint::ImageSource::OpaqueId`
- documented `# Panics`: false
- reproducer verified: true
- miri: not run (only panics are checked)
- rfuzz seed: 15326083453805907864

## Message

```
process ended abnormally (out of memory) while calling vello_common::paint::ImageSource::OpaqueId
```

## Reproducer

`Cargo.toml`:

```toml
[dependencies]
vello_common = { git = "https://github.com/linebender/vello", rev = "9cbe466bded9fff6cf7c1dd74246e95fe74f33c0" }
```

`src/main.rs`:

```rust
use vello_common::image_cache::ImageCache;
use vello_common::multi_atlas::AllocationStrategy;
use vello_common::multi_atlas::AtlasConfig;
use vello_common::paint::ImageSource;

fn main() {
    let _ = ImageSource::OpaqueId { id: ImageCache::new_with_config(AtlasConfig { initial_atlas_count: 1000000000usize, max_atlases: 1000000000usize, atlas_size: (1u16, 1u16), auto_grow: false, allocation_strategy: AllocationStrategy::BestFit }).allocate(0u16, 0u16, 0u16).unwrap(), may_have_transparency: false };
}
```

Run with `cargo run` in this directory (package `repro_out_of_memory_in_vello_common_paint_imagesource_opaqueid_190f3c`).

## Output

```
memory allocation of 32 bytes failed
memory allocation of 512 bytes failed
skipping backtrace printing to avoid potential recursion
```
